//! Blindly Loyal Goblin
use super::prelude::*;

pub const ID: u32 = 35215622;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // control
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EFFECT_CANNOT_CHANGE_CONTROL);
    card::register_effect(d, c, e1, false);
}
