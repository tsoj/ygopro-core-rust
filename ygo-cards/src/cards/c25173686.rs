//! Straight Flush
use super::prelude::*;

pub const ID: u32 = 25173686;

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
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // local seq=c:GetSequence()
    let seq = card::get_sequence(d, c);
    // if Duel.IsDuelType(DUEL_3_COLUMNS_FIELD) then
    if duel::is_duel_type(d, DUEL_3_COLUMNS_FIELD as u64) {
        // return seq>0 and seq<4
        seq > 0 && seq < 4
    } else {
        // return seq<5
        seq < 5
    }
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetMatchingGroupCount(s.filter,tp,0,LOCATION_SZONE,nil)==(Duel.IsDuelType(DUEL_3_COLUMNS_FIELD) and 3 or 5)
    let n = duel::get_matching_group_count(d, Some(&filter), a.tp, 0, LOCATION_SZONE, NO_EX);
    let want = if duel::is_duel_type(d, DUEL_3_COLUMNS_FIELD as u64) { 3 } else { 5 };
    n == want
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,0,LOCATION_SZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, 0, LOCATION_SZONE, 1, NO_EX);
    }
    // local sg=Duel.GetMatchingGroup(s.filter,tp,0,LOCATION_SZONE,nil)
    let sg = duel::get_matching_group(d, Some(&filter), a.tp, 0, LOCATION_SZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,sg,#sg,0,0)
    let n = sg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(sg), n, 0, 0);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local sg=Duel.GetMatchingGroup(s.filter,tp,0,LOCATION_SZONE,nil)
    let sg = duel::get_matching_group(d, Some(&filter), a.tp, 0, LOCATION_SZONE, NO_EX);
    // Duel.Destroy(sg,REASON_EFFECT)
    duel::destroy(d, sg, REASON_EFFECT, None, None);
}
