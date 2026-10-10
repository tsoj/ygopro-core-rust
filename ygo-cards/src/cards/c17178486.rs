//! Life Equalizer
use super::prelude::*;

pub const ID: u32 = 17178486;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_lp(d, a.tp) - duel::get_lp(d, 1 - a.tp) >= 8000 || duel::get_lp(d, 1 - a.tp) - duel::get_lp(d, a.tp) >= 8000
}
fn activate(d: &mut Duel, a: &EffArgs) {
    duel::set_lp(d, a.tp, 3000);
    duel::set_lp(d, 1 - a.tp, 3000);
}
