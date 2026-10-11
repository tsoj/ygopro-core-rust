//! Parthian Shot
use super::prelude::*;

pub const ID: u32 = 29185231;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMING_BATTLE_END)
    effect::set_hint_timing(d, e1, TIMING_BATTLE_END, None);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`: `return Duel.GetCurrentPhase()==PHASE_BATTLE`
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::get_current_phase(d) == PHASE_BATTLE
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, _a: &EffArgs) {
    // local turnp=Duel.GetTurnPlayer()
    let turnp = duel::get_turn_player(d);
    // Duel.SkipPhase(turnp,PHASE_BATTLE,RESET_PHASE|PHASE_END,1,1)
    duel::skip_phase(d, turnp, PHASE_BATTLE, RESET_PHASE | PHASE_END, 1, 1);
    // Duel.SkipPhase(turnp,PHASE_MAIN2,RESET_PHASE|PHASE_END,1)
    duel::skip_phase(d, turnp, PHASE_MAIN2, RESET_PHASE | PHASE_END, 1, 0);
}
