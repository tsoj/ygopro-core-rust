//! Spiritual Swords of Revealing Light
use super::prelude::*;

pub const ID: u32 = 78625592;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_ATTACK));
    card::register_effect(d, c, e1, false);
    // disable attack
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_condition(d, e2, Some(condition(cond)));
    effect::set_cost(d, e2, Some(cost_target(costf)));
    effect::set_operation(d, e2, Some(operation(op)));
    card::register_effect(d, c, e2, false);
    // cannot direct attack
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e3, 0, Some(TIMING_ATTACK));
    effect::set_range(d, e3, LOCATION_GRAVE);
    effect::set_condition(d, e3, Some(condition(grcondition)));
    effect::set_cost(d, e3, Some(cost::self_banish()));
    effect::set_operation(d, e3, Some(operation(groperation)));
    card::register_effect(d, c, e3, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp)
}
fn costf(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::check_lp_cost(d, a.tp, 1000);
    }
    duel::pay_lp_cost(d, a.tp, 1000);
    effect::set_label(d, a.e, &[1]);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    if effect::get_label(d, a.e) == 0 {
        return;
    }
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    duel::negate_attack(d);
}
fn grcondition(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp) && (duel::is_able_to_enter_bp(d) || duel::is_battle_phase(d, None))
}
fn groperation(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CANNOT_DIRECT_ATTACK);
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
}
