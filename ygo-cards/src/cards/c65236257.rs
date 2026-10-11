//! Tellarknight Genesis
use super::prelude::*;

pub const ID: u32 = 65236257;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TELLARKNIGHT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE|TIMING_EQUIP)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE | TIMING_EQUIP));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    // c:IsSpellTrap()
    card::is_spell_trap(d, c)
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // c:IsSetCard(SET_TELLARKNIGHT) and c:IsFaceup()
    card::is_set_card(d, c, &[SET_TELLARKNIGHT as u16]) && card::is_faceup(d, c)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ct=Duel.GetMatchingGroupCount(s.cfilter,tp,LOCATION_ONFIELD,0,c)
    let ct = duel::get_matching_group_count(d, Some(&cfilter), tp, LOCATION_ONFIELD, 0, c.into()) as u32;
    // if chk==0 then return ct>0
    //   and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,ct,c) end
    if a.chk == Some(0) {
        return ct > 0 && duel::is_existing_matching_card(d, Some(&filter), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, ct, c.into());
    }
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,c)
    let g = duel::get_matching_group(d, Some(&filter), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, c.into());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,ct,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), ct as u8, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ct=Duel.GetMatchingGroupCount(s.cfilter,tp,LOCATION_ONFIELD,0,c)
    let ct = duel::get_matching_group_count(d, Some(&cfilter), tp, LOCATION_ONFIELD, 0, c.into()) as u32;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,ct,ct,c)
    let g = duel::select_matching_card(d, tp, Some(&filter), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, ct as u16, ct as u16, false, c.into()).nil();
    // Duel.Destroy(g,REASON_EFFECT)
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
