//! Astral Barrier
use super::prelude::*;

pub const ID: u32 = 37053871;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END));
    card::register_effect(d, c, e1, false);
    // target change
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_type(d, e2, EFFECT_TYPE_FIELD | EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_condition(d, e2, Some(condition(atkcon)));
    effect::set_target(d, e2, Some(cost_target(atktg2)));
    effect::set_operation(d, e2, Some(operation(atkop)));
    card::register_effect(d, c, e2, false);
}
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let attacker = duel::get_attacker(d).nil();
    let at = duel::get_attack_target(d);
    if !card::is_controler(d, attacker, 1 - a.tp) {
        return false;
    }
    match at {
        Some(at) => card::is_controler(d, at, a.tp),
        None => false,
    }
}
fn atktg2(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let attacker = duel::get_attacker(d).nil();
        return card::is_has_effect(d, attacker, EFFECT_CANNOT_DIRECT_ATTACK, None).is_empty();
    }
    true
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    duel::change_attack_target(d, None, false);
}
