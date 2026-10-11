//! Windstorm of Etaqua
use super::prelude::*;

pub const ID: u32 = 59744639;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_POSITION)
    effect::set_category(d, e1, CATEGORY_POSITION);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER|TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_MAIN_END));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsCanChangePosition()
    card::is_faceup(d, c) && card::is_can_change_position(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // local sg=Duel.GetMatchingGroup(s.filter,tp,0,LOCATION_MZONE,nil)
    let sg = duel::get_matching_group(d, Some(&filter), tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,sg,#sg,0,0)
    let n = sg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(sg), n, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local sg=Duel.GetMatchingGroup(s.filter,tp,0,LOCATION_MZONE,nil)
    let sg = duel::get_matching_group(d, Some(&filter), tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.ChangePosition(sg,POS_FACEUP_DEFENSE,0,POS_FACEUP_ATTACK,0)
    duel::change_position(d, sg, POS_FACEUP_DEFENSE as u8, Some(0), Some(POS_FACEUP_ATTACK as u8), Some(0), false);
}
