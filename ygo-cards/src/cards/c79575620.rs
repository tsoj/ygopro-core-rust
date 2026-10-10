//! Injection Fairy Lily
use super::prelude::*;

pub const ID: u32 = 79575620;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // attack up
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EVENT_PRE_DAMAGE_CALCULATE);
    effect::set_condition(d, e1, Some(condition(con)));
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn con(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, c, ID) == 0 && (duel::get_attacker(d) == Some(c) || duel::get_attack_target(d) == Some(c))
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::check_lp_cost(d, tp, 2000);
    }
    duel::pay_lp_cost(d, tp, 2000);
    let c = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, c, ID, RESET_PHASE | PHASE_DAMAGE_CAL, 0, 1, 0, 0);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE_CAL, 1);
    effect::set_value(d, e1, 3000);
    card::register_effect(d, c, e1, false);
}
