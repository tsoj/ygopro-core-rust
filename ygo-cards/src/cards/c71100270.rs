//! Goblin's Crazy Beast
use super::prelude::*;

pub const ID: u32 = 71100270;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GOBLIN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // 2+ Level 6 monsters
    // Xyz.AddProcedure(c,nil,6,2,nil,nil,Xyz.InfiniteMats)
    xyz::add_procedure(d, c, None, Some(6), 2, None, None, Some(xyz::INFINITE_MATS), None, false, None);
    // Detach 2 materials from a monster(s) on the field
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E as u32));
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_target(d, e1, Some(cost_target(dttg)));
    effect::set_operation(d, e1, Some(operation(dtop)));
    card::register_effect(d, c, e1, false);
    // Destruction replacement
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_target(d, e2, Some(cost_target(reptg)));
    effect::set_operation(d, e2, Some(operation(repop)));
    effect::set_value_fn(d, e2, effect_card_fn(|d, e, x| {
        let tp = effect::get_handler_player(d, e);
        repfilter(d, x, tp)
    }));
    card::register_effect(d, c, e2, false);
    // Special Summon this card
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e3, LOCATION_GRAVE);
    effect::set_count_limit(d, e3, 1, ID, 0, EFFECT_COUNT_CODE_DUEL as u8);
    effect::set_target(d, e3, Some(cost_target(sptg)));
    effect::set_operation(d, e3, Some(operation(spop)));
    card::register_effect(d, c, e3, false);
}
/// `s.stfilter(c,xc,tp)`
fn stfilter(d: &mut Duel, c: CardId, xc: CardId, tp: u8) -> bool {
    card::is_spell_trap(d, c) && card::is_can_be_xyz_material(d, c, Some(xc), Some(tp), Some(REASON_EFFECT))
}
/// `s.dttg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn dttg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc)
            && card::is_spell_trap(d, chkc)
            && card::is_can_be_xyz_material(d, chkc, Some(c), Some(a.tp), Some(REASON_EFFECT));
    }
    if a.chk == Some(0) {
        // return Duel.CheckRemoveOverlayCard(tp,1,1,2,REASON_EFFECT)
        //     and Duel.IsExistingTarget(s.stfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil,c,tp)
        return duel::check_remove_overlay_card(d, a.tp, 1, 1, 2, REASON_EFFECT, None)
            && duel::is_existing_target(d, Some(&|d, x| stfilter(d, x, c, a.tp)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATTACH)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATTACH as u64);
    // Duel.SelectTarget(tp,s.stfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil,c,tp)
    let _g = duel::select_target(d, a.tp, Some(&|d, x| stfilter(d, x, c, a.tp)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    true
}
/// `s.dtop(e,tp,eg,ep,ev,re,r,rp)`
fn dtop(d: &mut Duel, a: &EffArgs) {
    // if Duel.RemoveOverlayCard(tp,1,1,2,2,REASON_EFFECT)==0 then return end
    if duel::remove_overlay_card(d, a.tp, 1, 1, 2, 2, REASON_EFFECT, None) == 0 {
        return;
    }
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and tc:IsRelateToEffect(e) and not tc:IsImmuneToEffect(e)
    //     and tc:IsCanBeXyzMaterial(c,tp,REASON_EFFECT) then
    if card::is_relate_to_effect(d, c, a.e) {
        let tc = tc.nil();
        if card::is_relate_to_effect(d, tc, a.e)
            && !card::is_immune_to_effect(d, tc, a.e)
            && card::is_can_be_xyz_material(d, tc, Some(c), Some(a.tp), Some(REASON_EFFECT))
        {
            // Duel.Overlay(c,tc)
            duel::overlay(d, c, tc, false);
        }
    }
}
/// `s.repfilter(c,tp)`
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_location(d, c, LOCATION_MZONE)
        && card::is_controler(d, c, tp)
        && card::is_reason(d, c, REASON_EFFECT)
        && !card::is_reason(d, c, REASON_REPLACE)
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        // return eg:IsExists(s.repfilter,1,nil,tp) and c:CheckRemoveOverlayCard(tp,1,REASON_EFFECT)
        let eg = a.eg.clone().nil();
        return group::is_exists(d, &eg, &|d, x| repfilter(d, x, a.tp), 1, NO_EX)
            && card::check_remove_overlay_card(d, c, a.tp, 1, REASON_EFFECT);
    }
    // return Duel.SelectEffectYesNo(tp,c,96)
    duel::select_effect_yes_no(d, a.tp, c, Some(96))
}
/// `function(e,tp) e:GetHandler():RemoveOverlayCard(tp,1,1,REASON_EFFECT) end`
fn repop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    card::remove_overlay_card(d, c, a.tp, 1, 1, REASON_EFFECT);
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_LEAVE_GRAVE,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.gfilter(c,xc,tp)`
fn gfilter(d: &mut Duel, c: CardId, xc: CardId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_GOBLIN as u16]) && card::is_monster(d, c) && card::is_can_be_xyz_material(d, c, Some(xc), Some(tp), Some(REASON_EFFECT))
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0
    //     and Duel.IsExistingMatchingCard(s.gfilter,tp,LOCATION_GRAVE,0,1,nil,c,tp)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if card::is_relate_to_effect(d, c, a.e)
        && duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0
        && duel::is_existing_matching_card(d, Some(&|d, x| gfilter(d, x, c, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATTACH)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATTACH as u64);
        // local g=Duel.SelectMatchingCard(tp,s.gfilter,tp,LOCATION_GRAVE,0,1,1,nil,c,tp)
        let g = duel::select_matching_card(d, a.tp, Some(&|d, x| gfilter(d, x, c, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // if #g==0 then return end
        if g.is_empty() {
            return;
        }
        // Duel.HintSelection(g,true)
        duel::hint_selection(d, g.clone(), true);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Overlay(c,g)
        duel::overlay(d, c, g, false);
    }
}
