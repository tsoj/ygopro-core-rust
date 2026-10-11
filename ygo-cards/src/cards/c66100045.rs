//! Transmigration Break
use super::prelude::*;

pub const ID: u32 = 66100045;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // remove
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_target_range(d, e2, LOCATION_GRAVE, LOCATION_GRAVE);
    effect::set_code(d, e2, EFFECT_TO_DECK_REDIRECT);
    effect::set_value(d, e2, LOCATION_REMOVED as i64);
    card::register_effect(d, c, e2, false);
}
