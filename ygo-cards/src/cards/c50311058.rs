//! Goblin Biker Grand Stampede
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 50311058;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GOBLIN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Xyz Summon using only "Goblin" monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER|TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_MAIN_END));
    // e1:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e1, Some(condition(cond_main)));
    // e1:SetTarget(s.xyztg)
    effect::set_target(d, e1, Some(cost_target(xyztg)));
    // e1:SetOperation(s.xyzop)
    effect::set_operation(d, e1, Some(operation(xyzop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Detach 1 Xyz material from a monster on the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e2, CATEGORY_DISABLE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E|TIMING_MAIN_END)
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER_E | TIMING_MAIN_END));
    // e2:SetCondition(function(_,tp) return Duel.IsExistingMatchingCard(s.gfilter,tp,LOCATION_MZONE,0,1,nil) end)
    effect::set_condition(d, e2, Some(condition(cond_gfilter)));
    // e2:SetTarget(s.dttg)
    effect::set_target(d, e2, Some(cost_target(dttg)));
    // e2:SetOperation(s.dtop)
    effect::set_operation(d, e2, Some(operation(dtop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function() return Duel.IsMainPhase() end`
fn cond_main(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_main_phase(d, None)
}
/// `function(_,tp) return Duel.IsExistingMatchingCard(s.gfilter,tp,LOCATION_MZONE,0,1,nil) end`
fn cond_gfilter(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&gfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.matfilter(c)`
fn matfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_GOBLIN) and c:IsCanBeXyzMaterial()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_GOBLIN as u16]) && card::is_can_be_xyz_material(d, c, None, None, None)
}
/// `s.xyztg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn xyztg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local mg=Duel.GetMatchingGroup(s.matfilter,tp,LOCATION_MZONE,0,nil)
        let mg = duel::get_matching_group(d, Some(&matfilter), tp, LOCATION_MZONE, 0, NO_EX);
        // return Duel.IsExistingMatchingCard(Card.IsXyzSummonable,tp,LOCATION_EXTRA,0,1,nil,nil,mg)
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_xyz_summonable(d, c, None, Some(mg.clone()), 0, 0)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.xyzop(e,tp,eg,ep,ev,re,r,rp)`
fn xyzop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local mg=Duel.GetMatchingGroup(s.matfilter,tp,LOCATION_MZONE,0,nil)
    let mg = duel::get_matching_group(d, Some(&matfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // if #mg==0 then return end
    if mg.is_empty() {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local xc=Duel.SelectMatchingCard(tp,Card.IsXyzSummonable,tp,LOCATION_EXTRA,0,1,1,nil,nil,mg):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, c| card::is_xyz_summonable(d, c, None, Some(mg.clone()), 0, 0)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let xc = group::get_first(d, &sel);
    // if xc then
    if let Some(xc) = xc {
        // Duel.XyzSummon(tp,xc,nil,mg)
        duel::xyz_summon(d, tp, xc, None, Some(mg), 0, 0);
    }
}
/// `s.gfilter(c)`
fn gfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_XYZ) and c:IsSetCard(SET_GOBLIN)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ) && card::is_set_card(d, c, &[SET_GOBLIN as u16])
}
/// `s.disfilter(c)`
fn disfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsNegatableMonster() and c:IsType(TYPE_EFFECT)
    card::is_negatable_monster(d, c) && card::is_type(d, c, TYPE_EFFECT)
}
/// `s.dttg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn dttg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.disfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && disfilter(d, chkc);
    }
    // if chk==0 then return Duel.CheckRemoveOverlayCard(tp,1,1,1,REASON_EFFECT)
    //     and Duel.IsExistingTarget(s.disfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::check_remove_overlay_card(d, tp, 1, 1, 1, REASON_EFFECT, None)
            && duel::is_existing_target(d, Some(&disfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NEGATE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_NEGATE as u64);
    // local g=Duel.SelectTarget(tp,s.disfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&disfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, g, 1, 0, 0);
    true
}
/// `s.dtop(e,tp,eg,ep,ev,re,r,rp)`
fn dtop(d: &mut Duel, a: &EffArgs) {
    // if Duel.RemoveOverlayCard(tp,1,1,1,1,REASON_EFFECT)==0 then return end
    if duel::remove_overlay_card(d, a.tp, 1, 1, 1, 1, REASON_EFFECT, None) == 0 {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // tc:NegateEffects(e:GetHandler(),RESET_PHASE|PHASE_END)
        let h = effect::get_handler(d, a.e).nil();
        card::negate_effects(d, tc, h, Some(RESET_PHASE | PHASE_END), false, None);
    }
}
