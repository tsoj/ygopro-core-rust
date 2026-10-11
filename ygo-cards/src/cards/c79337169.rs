//! Gauntlet Warrior
use super::prelude::*;

pub const ID: u32 = 79337169;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atkup
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    effect::set_condition(d, e1, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    effect::set_cost(d, e1, Some(cost::self_tribute()));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    card::register_effect(d, c, e1, false);
}
fn warrior_filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_race(d, c, RACE_WARRIOR)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&warrior_filter), a.tp, LOCATION_MZONE, 0, 1, c.into());
    }
    true
}
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, Some(&warrior_filter), a.tp, LOCATION_MZONE, 0, c.into());
    for tc in group::iter(d, &g) {
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_value(d, e1, 500);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e1, false);
        let e2 = effect::clone(d, e1);
        effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
        effect::set_label_object(d, e2, Value::Effect(e1));
        card::register_effect(d, tc, e2, false);
        let e3 = effect::create_effect(d, c);
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE | EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, e3, EVENT_DAMAGE_STEP_END);
        effect::set_operation(d, e3, Some(operation(resetop)));
        effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
        effect::set_label_object(d, e3, Value::Effect(e2));
        card::register_effect(d, tc, e3, false);
    }
}
fn resetop(d: &mut Duel, a: &EffArgs) {
    let e1 = effect::get_label_object(d, a.e).as_effect().nil();
    let e2 = effect::get_label_object(d, e1).as_effect().nil();
    effect::reset(d, e1);
    effect::reset(d, e2);
    effect::reset(d, a.e);
}
