//! A Legendary Ocean
use super::prelude::*;

pub const ID: u32 = 295517;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // level
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_UPDATE_LEVEL);
    effect::set_range(d, e2, LOCATION_FZONE);
    effect::set_target_range(d, e2, LOCATION_HAND | LOCATION_MZONE, LOCATION_HAND | LOCATION_MZONE);
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_attribute(d, c, ATTRIBUTE_WATER))));
    effect::set_value(d, e2, -1);
    card::register_effect(d, c, e2, false);
    // Atk
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_value(d, e3, 200);
    card::register_effect(d, c, e3, false);
    // Def
    let e4 = effect::clone(d, e3);
    effect::set_code(d, e4, EFFECT_UPDATE_DEFENSE);
    card::register_effect(d, c, e4, false);
}
