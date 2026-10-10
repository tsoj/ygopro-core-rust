//! Gora Turtle
use super::prelude::*;

pub const ID: u32 = 80233946;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK_ANNOUNCE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetTarget(s.atktarget)
    effect::set_target(d, e1, Some(effect_card_fn(atktarget)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.atktarget(e,c)`
fn atktarget(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsAttackAbove(1900)
    card::is_attack_above(d, c, 1900)
}
