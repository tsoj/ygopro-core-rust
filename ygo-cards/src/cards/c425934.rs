//! Goblin of Greed
use super::prelude::*;

pub const ID: u32 = 425934;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // discard limit
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target_range(d, e1, 1, 1);
    effect::set_code(d, e1, EFFECT_CANNOT_DISCARD_HAND);
    effect::set_value(d, e1, 1);
    card::register_effect(d, c, e1, false);
}
