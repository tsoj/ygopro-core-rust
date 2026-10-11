//! Pilgrim of the Ice Barrier
use super::prelude::*;

pub const ID: u32 = 20700531;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // indes
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    effect::set_value_fn(d, e1, effect_card_fn(indes));
    card::register_effect(d, c, e1, false);
}
fn indes(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::get_attack(d, c) >= 1900
}
