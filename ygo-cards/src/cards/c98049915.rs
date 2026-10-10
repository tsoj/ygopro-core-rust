//! Mystic Lamp
use super::prelude::*;

pub const ID: u32 = 98049915;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // direct attack
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
    card::register_effect(d, c, e1, false);
}
