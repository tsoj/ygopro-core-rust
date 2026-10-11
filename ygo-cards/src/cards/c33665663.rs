//! Fire Formation - Kaiyo
use super::prelude::*;

pub const ID: u32 = 33665663;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // atkup
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_target(d, e2, Some(effect_card_fn(beast_warrior)));
    effect::set_value(d, e2, 300);
    card::register_effect(d, c, e2, false);
}
fn beast_warrior(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_race(d, c, RACE_BEASTWARRIOR as u64)
}
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    aux::stat_change_damage_step_condition(d)
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_PIERCE);
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_target(d, e1, Some(effect_card_fn(beast_warrior)));
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
}
