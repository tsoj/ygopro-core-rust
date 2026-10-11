//! Flashbang
use super::prelude::*;

pub const ID: u32 = 9267769;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e1, EVENT_BATTLE_DAMAGE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:GetFirst():IsControler(1-tp) and Duel.GetAttackTarget()==nil
    let eg = a.eg.clone().nil();
    let tc = group::get_first(d, &eg).nil();
    card::is_controler(d, tc, 1 - a.tp) && duel::get_attack_target(d).is_none()
}
// s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // Duel.SkipPhase(1-tp,PHASE_BATTLE,RESET_PHASE|PHASE_END,1,1)
    duel::skip_phase(d, 1 - a.tp, PHASE_BATTLE, RESET_PHASE | PHASE_END, 1, 1);
    // Duel.SkipPhase(1-tp,PHASE_MAIN2,RESET_PHASE|PHASE_END,1)
    duel::skip_phase(d, 1 - a.tp, PHASE_MAIN2, RESET_PHASE | PHASE_END, 1, 0);
}
