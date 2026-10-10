//! Amazoness Swords Woman
use super::prelude::*;

pub const ID: u32 = 94004268;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // reflect battle dam
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_REFLECT_BATTLE_DAMAGE);
    effect::set_value(d, e1, 1);
    card::register_effect(d, c, e1, false);
}
