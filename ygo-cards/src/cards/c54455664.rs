//! Dragunity Brandistock
use super::prelude::*;

pub const ID: u32 = 54455664;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Double Attack
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
    effect::set_value(d, e1, 1);
    card::register_effect(d, c, e1, false);
}
