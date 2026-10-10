//! Chain Energy
use super::prelude::*;

pub const ID: u32 = 79323590;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // activate cost
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_ACTIVATE_COST);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e2, 1, 1);
    effect::set_target(d, e2, Some(lua_fn(actarget)));
    effect::set_cost(d, e2, Some(lua_fn(costchk)));
    effect::set_operation(d, e2, Some(operation(costop)));
    card::register_effect(d, c, e2, false);
    // summon cost
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_code(d, e3, EFFECT_SUMMON_COST);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_target_range(d, e3, LOCATION_HAND, LOCATION_HAND);
    effect::set_cost(d, e3, Some(lua_fn(costchk)));
    effect::set_operation(d, e3, Some(operation(costop)));
    card::register_effect(d, c, e3, false);
    let e4 = effect::clone(d, e3);
    effect::set_code(d, e4, EFFECT_SPSUMMON_COST);
    card::register_effect(d, c, e4, false);
    // set cost
    let e5 = effect::clone(d, e3);
    effect::set_code(d, e5, EFFECT_MSET_COST);
    card::register_effect(d, c, e5, false);
    let e6 = effect::clone(d, e3);
    effect::set_code(d, e6, EFFECT_SSET_COST);
    card::register_effect(d, c, e6, false);
    // accumulate
    let e7 = effect::create_effect(d, c);
    effect::set_type(d, e7, EFFECT_TYPE_FIELD);
    effect::set_code(d, e7, ID);
    effect::set_range(d, e7, LOCATION_SZONE);
    effect::set_property(d, e7, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e7, 1, 1);
    card::register_effect(d, c, e7, false);
}
fn actarget(d: &mut Duel, a: Args) -> bool {
    let te = a.e(1);
    let c = effect::get_handler(d, te).nil();
    card::is_location(d, c, LOCATION_HAND)
}
fn costchk(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(2);
    let ct = duel::get_player_effect(d, tp, ID).len() as u32;
    duel::check_lp_cost(d, tp, ct * 500)
}
fn costop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    duel::pay_lp_cost(d, a.tp, 500);
}
