//! Mad Sword Beast
use super::prelude::*;

pub const ID: u32 = 79870141;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // pierce
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_PIERCE);
    card::register_effect(d, c, e1, false);
}
