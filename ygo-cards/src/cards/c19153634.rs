//! Patrician of Darkness
use super::prelude::*;

pub const ID: u32 = 19153634;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EFFECT_PATRICIAN_OF_DARKNESS);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e1, 0, 1);
    card::register_effect(d, c, e1, false);
}
