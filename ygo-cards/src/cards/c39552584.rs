//! Grapha, Dragon Overlord of Dark World
use super::prelude::*;

pub const ID: u32 = 39552584;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[34230233], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,34230233,aux.FilterBoolFunctionEx(Card.IsAttribute,ATTRIBUTE_DARK))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::Code(34230233),
            fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| card::is_attribute_ctx(d, x, ATTRIBUTE_DARK, Some(fc), st, Some(tp)))),
        ],
    );
    // Change the effect of a Normal Spell/Trap card or monster
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(chcon)));
    effect::set_target(d, e1, Some(cost_target(chtg)));
    effect::set_operation(d, e1, Some(operation(chop)));
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "Grapha, Dragon Lord of Dark World"
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_HANDES);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_LEAVE_FIELD);
    effect::set_condition(d, e2, Some(condition(spcon)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
}
/// `s.chcon(e,tp,eg,ep,ev,re,r,rp)`
fn chcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, a.re.nil()).nil();
    // return rp==1-tp and (re:IsMonsterEffect() or ((rc:IsNormalSpell() or rc:IsNormalTrap()) and re:IsHasType(EFFECT_TYPE_ACTIVATE)))
    a.rp == 1 - a.tp
        && (effect::is_monster_effect(d, a.re.nil())
            || ((card::is_normal_spell(d, rc) || card::is_normal_trap(d, rc)) && effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE)))
}
/// `s.chtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn chtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0) > 0;
    }
    true
}
/// `s.chop(e,tp,eg,ep,ev,re,r,rp)`
fn chop(d: &mut Duel, a: &EffArgs) {
    // local g=Group.CreateGroup()
    let g = CardSet::new();
    // Duel.ChangeTargetCard(ev,g)
    duel::change_target_card(d, a.ev as u8, &g);
    // Duel.ChangeChainOperation(ev,s.repop)
    duel::change_chain_operation(d, a.ev as u8, operation(repop));
}
/// `s.repop(e,tp,eg,ep,ev,re,r,rp)`
fn repop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_HAND)
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_HAND);
    // if #g>0 then Duel.DiscardHand(1-tp,nil,1,1,REASON_EFFECT|REASON_DISCARD) end
    if !g.is_empty() {
        duel::discard_hand(d, 1 - a.tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return rp==1-tp and c:IsFusionSummoned() and c:IsPreviousLocation(LOCATION_MZONE) and c:IsPreviousControler(tp)
    a.rp == 1 - a.tp
        && card::is_fusion_summoned(d, c)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_controler(d, c, a.tp)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsCode(34230233) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_faceup(d, c) && card::is_code(d, c, &[34230233]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE|LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,PLAYER_ALL,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, PLAYER_ALL as u8, 1);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp))),
        a.tp,
        LOCATION_GRAVE | LOCATION_REMOVED,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if #g>0 and Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)>0
    //     and (Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)>0 or Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)>0) then
    if !g.is_empty()
        && duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0
        && (duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0) > 0 || duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND) > 0)
    {
        // local turn_pl=Duel.GetTurnPlayer()
        let turn_pl = duel::get_turn_player(d);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.DiscardHand(turn_pl,nil,1,1,REASON_EFFECT|REASON_DISCARD)
        duel::discard_hand(d, turn_pl, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
        // Duel.DiscardHand(1-turn_pl,nil,1,1,REASON_EFFECT|REASON_DISCARD)
        duel::discard_hand(d, 1 - turn_pl, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
    }
}
