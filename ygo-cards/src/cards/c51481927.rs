//! Spell Absorption
use super::prelude::*;

pub const ID: u32 = 51481927;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // LP up
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e0, EVENT_CHAINING);
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_range(d, e0, LOCATION_SZONE);
    effect::set_operation(d, e0, Some(aux::chainreg()));
    card::register_effect(d, c, e0, false);
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_CHAIN_SOLVED);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_condition(d, e2, Some(condition(cond)));
    effect::set_operation(d, e2, Some(operation(op)));
    card::register_effect(d, c, e2, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let Some(re) = a.re else { return false };
    if !(effect::is_spell_effect(d, re) && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)) {
        return false;
    }
    let c = effect::get_handler(d, a.e).nil();
    effect::get_handler(d, re) != Some(c) && card::get_flag_effect(d, c, 1) > 0
}
fn op(d: &mut Duel, a: &EffArgs) {
    let p = effect::get_handler_player(d, a.e);
    duel::recover(d, p, 500, REASON_EFFECT, false, None);
}
