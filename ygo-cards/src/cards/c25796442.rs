//! Ritual Cage
use super::prelude::*;

pub const ID: u32 = 25796442;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    //
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_AVOID_BATTLE_DAMAGE);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_type(d, c, TYPE_RITUAL))));
    effect::set_value(d, e2, 1);
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EFFECT_CANNOT_BE_EFFECT_TARGET);
    effect::set_property(d, e3, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_value_fn(d, e3, lua_fn(tgval));
    card::register_effect(d, c, e3, false);
    let e4 = effect::clone(d, e2);
    effect::set_code(d, e4, EFFECT_INDESTRUCTABLE_EFFECT);
    effect::set_value_fn(d, e4, lua_fn(efilter));
    card::register_effect(d, c, e4, false);
}
/// `s.tgval(e,re,rp)`
fn tgval(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    effect::is_active_type(d, re, TYPE_EFFECT)
}
/// `s.efilter(e,re)`
fn efilter(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    effect::is_active_type(d, re, TYPE_EFFECT)
}
