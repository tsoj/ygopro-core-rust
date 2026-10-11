//! Timidity
use super::prelude::*;

pub const ID: u32 = 40350910;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
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
    // indestructable
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    // e1:SetTargetRange(LOCATION_SZONE,LOCATION_SZONE)
    effect::set_target_range(d, e1, LOCATION_SZONE, LOCATION_SZONE);
    // e1:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE, 0);
    // e1:SetTarget(s.infilter)
    effect::set_target(d, e1, Some(effect_card_fn(infilter)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // e1:SetReset(RESET_PHASE|PHASE_END,2)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 2);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `s.infilter(e,c)`
fn infilter(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsFacedown()
    card::is_facedown(d, c)
}
