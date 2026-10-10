//! Self-Destruct Button
use super::prelude::*;

pub const ID: u32 = 57585212;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetLP(tp)<=Duel.GetLP(1-tp)-7000
    duel::get_lp(d, a.tp) <= duel::get_lp(d, 1 - a.tp) - 7000
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // Duel.SetLP(tp,0)
    duel::set_lp(d, a.tp, 0);
    // Duel.SetLP(1-tp,0)
    duel::set_lp(d, 1 - a.tp, 0);
}
