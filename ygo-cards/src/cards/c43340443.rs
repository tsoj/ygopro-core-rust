//! Forced Back
use super::prelude::*;

pub const ID: u32 = 43340443;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DISABLE_SUMMON+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_DISABLE_SUMMON + CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_SUMMON)
    effect::set_code(d, e1, EVENT_SUMMON);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_FLIP_SUMMON)
    effect::set_code(d, e2, EVENT_FLIP_SUMMON);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.GetCurrentChain(true)==0
    duel::get_current_chain(d, true) == 0
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE_SUMMON,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE_SUMMON, a.eg.clone(), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, a.eg.clone(), 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp,chk)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // Duel.NegateSummon(eg:GetFirst())
    let first = group::get_first(d, &a.eg.clone().nil()).nil();
    duel::negate_summon(d, first);
    // Duel.SendtoHand(eg,nil,REASON_EFFECT)
    duel::send_to_hand(d, a.eg(), None, REASON_EFFECT, None);
}
