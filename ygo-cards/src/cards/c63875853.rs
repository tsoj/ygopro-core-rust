//! Trading Places
//! 身分転換
use super::prelude::*;

pub const ID: u32 = 63875853;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLP(tp)>Duel.GetLP(1-tp) end
    if a.chk == Some(0) {
        return duel::get_lp(d, a.tp) > duel::get_lp(d, 1 - a.tp);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local lp1=Duel.GetLP(tp)
    let lp1 = duel::get_lp(d, a.tp);
    // local lp2=Duel.GetLP(1-tp)
    let lp2 = duel::get_lp(d, 1 - a.tp);
    // if lp1>lp2 then
    if lp1 > lp2 {
        // Duel.SetLP(tp,lp2)
        duel::set_lp(d, a.tp, lp2);
        // Duel.SetLP(1-tp,lp1)
        duel::set_lp(d, 1 - a.tp, lp1);
    }
}
