//! Insect Barrier
use super::prelude::*;

pub const ID: u32 = 23615409;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // cannot attack
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_CANNOT_ATTACK_ANNOUNCE);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    effect::set_target(d, e2, Some(effect_card_fn(atktarget)));
    card::register_effect(d, c, e2, false);
}
fn atktarget(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsRace(RACE_INSECT)
    card::is_race(d, c, RACE_INSECT)
}
