//! Mistaken Arrest
use super::prelude::*;

pub const ID: u32 = 4227096;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_TO_HAND)
    effect::set_code(d, e1, EFFECT_CANNOT_TO_HAND);
    // e1:SetTargetRange(LOCATION_DECK,LOCATION_DECK)
    effect::set_target_range(d, e1, LOCATION_DECK, LOCATION_DECK);
    // if Duel.IsTurnPlayer(tp) then
    if duel::is_turn_player(d, tp) {
        // e1:SetReset(RESET_PHASE|PHASE_END|RESET_SELF_TURN,2)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_SELF_TURN, 2);
    } else {
        // e1:SetReset(RESET_PHASE|PHASE_END|RESET_SELF_TURN)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_SELF_TURN, 1);
    }
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
