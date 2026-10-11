//! Dimension Fortress Weapon
use super::prelude::*;

pub const ID: u32 = 1596508;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Cards cannot be sent from the Deck to the GY
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CANNOT_TO_GRAVE);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target_range(d, e1, LOCATION_DECK, LOCATION_DECK);
    card::register_effect(d, c, e1, false);
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e2, EFFECT_CANNOT_DISCARD_DECK);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_target_range(d, e2, 1, 1);
    card::register_effect(d, c, e2, false);
}
