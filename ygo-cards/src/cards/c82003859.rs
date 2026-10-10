//! Toll
use super::prelude::*;

pub const ID: u32 = 82003859;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // attack cost
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_ATTACK_COST);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e2, 1, 1);
    effect::set_cost(d, e2, Some(lua_fn(atcost)));
    effect::set_operation(d, e2, Some(operation(atop)));
    card::register_effect(d, c, e2, false);
    // accumulate
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_code(d, e3, ID);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e3, 1, 1);
    card::register_effect(d, c, e3, false);
}
fn atcost(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(2);
    let ct = duel::get_player_effect(d, tp, ID).len();
    duel::check_lp_cost(d, tp, (ct * 500) as u32)
}
fn atop(d: &mut Duel, a: &EffArgs) {
    if duel::is_attack_cost_paid(d) != 2 {
        let h = effect::get_handler(d, a.e).nil();
        if card::is_location(d, h, LOCATION_SZONE) {
            duel::pay_lp_cost(d, a.tp, 500);
            duel::attack_cost_paid(d, None);
        }
    }
}
