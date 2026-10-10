//! Umi
use super::prelude::*;

pub const ID: u32 = 22702055;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Atk
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e2, LOCATION_FZONE);
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_value_fn(d, e2, effect_card_value(val));
    card::register_effect(d, c, e2, false);
    // Def
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    card::register_effect(d, c, e3, false);
}
fn val(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    let r = card::get_race(d, c);
    if r & (RACE_FISH | RACE_SEASERPENT | RACE_THUNDER | RACE_AQUA) > 0 {
        200
    } else if r & (RACE_MACHINE | RACE_PYRO) > 0 {
        -200
    } else {
        0
    }
}
