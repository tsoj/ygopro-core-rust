//! Underdog
use super::prelude::*;

pub const ID: u32 = 5779502;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Negate effects depending on turn player
    let e2 = effect::create_effect(d, c);
    effect::set_category(d, e2, CATEGORY_DISABLE);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_DISABLE);
    effect::set_condition(d, e2, Some(condition(con1)));
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_condition(d, e3, Some(condition(con2)));
    effect::set_target_range(d, e3, LOCATION_MZONE, 0);
    card::register_effect(d, c, e3, false);
}
fn con1(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_turn_player(d) == effect::get_handler_player(d, a.e)
}
fn con2(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_turn_player(d) != effect::get_handler_player(d, a.e)
}
