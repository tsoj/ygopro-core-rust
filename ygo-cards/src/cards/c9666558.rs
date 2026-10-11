//! Golem Dragon
use super::prelude::*;

pub const ID: u32 = 9666558;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot be battle target
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    effect::set_value_fn(d, e1, effect_card_fn(tg));
    card::register_effect(d, c, e1, false);
}
fn tg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    c != effect::get_handler(d, e).nil() && card::is_faceup(d, c) && card::is_race(d, c, RACE_DRAGON)
}
