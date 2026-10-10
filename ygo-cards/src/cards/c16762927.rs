//! Gravekeeper's Servant
use super::prelude::*;

pub const ID: u32 = 16762927;

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
    effect::set_target_range(d, e2, 0, 1);
    effect::set_cost(d, e2, Some(lua_fn(atcost)));
    effect::set_operation(d, e2, Some(operation(atop)));
    card::register_effect(d, c, e2, false);
    // accumulate
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_code(d, e3, ID);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e3, 0, 1);
    card::register_effect(d, c, e3, false);
}
/// `function(e,c,tp)`
fn atcost(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(2);
    // local ct=#{Duel.GetPlayerEffect(tp,id)}
    let ct = duel::get_player_effect(d, tp, ID).len();
    duel::is_player_can_discard_deck_as_cost(d, tp, ct as u32)
}
fn atop(d: &mut Duel, a: &EffArgs) {
    if duel::is_attack_cost_paid(d) != 2 {
        let h = effect::get_handler(d, a.e).nil();
        if card::is_location(d, h, LOCATION_SZONE) {
            duel::discard_deck(d, a.tp, 1, REASON_COST);
            duel::attack_cost_paid(d, None);
        }
    }
}
