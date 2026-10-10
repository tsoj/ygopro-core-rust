//! The Sanctuary in the Sky
use super::prelude::*;

pub const ID: u32 = 56433456;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // avoid battle damage
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_AVOID_BATTLE_DAMAGE);
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_range(d, e2, LOCATION_FZONE);
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_race(d, c, RACE_FAIRY))));
    effect::set_value(d, e2, 1);
    card::register_effect(d, c, e2, false);
}
