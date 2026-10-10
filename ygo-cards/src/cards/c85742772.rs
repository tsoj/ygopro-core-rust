//! Gravity Bind
use super::prelude::*;

pub const ID: u32 = 85742772;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // cannot attack
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_CANNOT_ATTACK);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_target(d, e2, Some(effect_card_fn(atktarget)));
    card::register_effect(d, c, e2, false);
}
/// `s.atktarget(e,c)`
fn atktarget(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:GetLevel()>=4
    card::get_level(d, c) >= 4
}
