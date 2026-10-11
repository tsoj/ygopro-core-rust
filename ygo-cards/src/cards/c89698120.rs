//! Tactical Espionage Expert
use super::prelude::*;

pub const ID: u32 = 89698120;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // summon success
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetOperation(s.sumsuc)
    effect::set_operation(d, e1, Some(operation(sumsuc)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.sumsuc(e,tp,eg,ep,ev,re,r,rp)
fn sumsuc(d: &mut Duel, _a: &EffArgs) {
    // Duel.SetChainLimitTillChainEnd(s.chlimit)
    duel::set_chain_limit_till_chain_end(d, lua_fn(chlimit));
}
/// `function s.chlimit(re,rp,tp)`
fn chlimit(d: &mut Duel, a: Args) -> bool {
    let re = a.e(0);
    // return not re:GetHandler():IsTrap() or not re:IsHasType(EFFECT_TYPE_ACTIVATE)
    let h = effect::get_handler(d, re).nil();
    !card::is_trap(d, h) || !effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
}
