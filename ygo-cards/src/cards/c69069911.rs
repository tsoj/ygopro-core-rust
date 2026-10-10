//! Kachi Kochi Dragon
use super::prelude::*;

pub const ID: u32 = 69069911;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    xyz::add_procedure(d, c, None, Some(4), 2, None, None, None, None, false, None);
    card::enable_revive_limit(d, c);
    // chain attack
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYING);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(atcon)));
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    effect::set_operation(d, e1, Some(operation(atop)));
    card::register_effect(d, c, e1, false);
}
fn atcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, c).nil();
    card::is_location(d, bc, LOCATION_GRAVE) && card::is_monster(d, bc) && card::can_chain_attack(d, c, None, false) && card::is_status(d, c, STATUS_OPPO_BATTLE)
}
fn atop(d: &mut Duel, _a: &EffArgs) {
    duel::chain_attack(d, None);
}
