//! Spell Reclamation
use super::prelude::*;

pub const ID: u32 = 76137276;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==tp and re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:IsSpellEffect()
    a.rp == a.tp && effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE) && effect::is_spell_effect(d, a.re.nil())
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
    let rh = effect::get_handler(d, a.re.nil()).nil();
    if card::is_relate_to_effect(d, rh, a.re.nil()) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let c = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, c);
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
    // if not e:GetHandler():IsHasEffect(EFFECT_NECRO_VALLEY) then
    let h = effect::get_handler(d, a.e).nil();
    if card::is_has_effect(d, h, EFFECT_NECRO_VALLEY, None).is_empty() {
        // Duel.SendtoHand(e:GetHandler(),tp,REASON_EFFECT)
        let h2 = effect::get_handler(d, a.e).nil();
        duel::send_to_hand(d, h2, Some(a.tp), REASON_EFFECT, None);
    }
}
