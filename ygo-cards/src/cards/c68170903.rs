//! A Feint Plan
//! 陽動作戦
use super::prelude::*;

pub const ID: u32 = 68170903;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_BATTLE_START)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_BATTLE_START));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetValue(s.atlimit)
    effect::set_value_fn(d, e1, effect_card_fn(atlimit));
    // e1:SetReset(RESET_PHASE|PHASE_END,1)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `s.atlimit(e,c)`
fn atlimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsFacedown()
    card::is_facedown(d, c)
}
