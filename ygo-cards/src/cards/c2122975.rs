//! Trap Reclamation
use super::prelude::*;

pub const ID: u32 = 2122975;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    // e1:SetCode(EVENT_CHAINING)
    // e1:SetCondition(s.condition)
    // e1:SetCost(s.cost)
    // e1:SetOperation(s.activate)
    // c:RegisterEffect(e1)
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==tp and re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:IsTrapEffect()
    let re = a.re.nil();
    a.rp == a.tp && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && effect::is_trap_effect(d, re)
}
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, None, None)), tp, LOCATION_HAND, 0, 1, h.into());
    }
    // Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_COST|REASON_DISCARD,nil)
    duel::discard_hand(d, tp, Some(&|d, c| card::is_discardable(d, c, None, None)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // if re:GetHandler():IsRelateToEffect(re) then
    let re = a.re.nil();
    let rh = effect::get_handler(d, re).nil();
    if card::is_relate_to_effect(d, rh, re) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_TO_GRAVE)
        effect::set_code(d, e1, EVENT_TO_GRAVE);
        // e1:SetOperation(s.thop)
        effect::set_operation(d, e1, Some(operation(thop)));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_EXC_GRAVE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_EXC_GRAVE, 1);
        // re:GetHandler():RegisterEffect(e1)
        card::register_effect(d, rh, e1, false);
    }
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    // if not e:GetHandler():IsHasEffect(EFFECT_NECRO_VALLEY) then
    if card::is_has_effect(d, h, EFFECT_NECRO_VALLEY, None).is_empty() {
        // Duel.SendtoHand(e:GetHandler(),tp,REASON_EFFECT)
        duel::send_to_hand(d, h, Some(tp), REASON_EFFECT, None);
    }
}
