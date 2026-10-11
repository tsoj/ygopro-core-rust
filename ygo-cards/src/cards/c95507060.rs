//! Raging Mad Plants
use super::prelude::*;

pub const ID: u32 = 95507060;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    effect::set_condition(d, e1, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_PLANT)), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| card::is_race(d, c, RACE_PLANT)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_PLANT)), tp, LOCATION_MZONE, 0, NO_EX);
    let atk = duel::get_matching_group_count(d, Some(&|d, c| card::is_race(d, c, RACE_PLANT)), tp, LOCATION_GRAVE, 0, NO_EX) as i64 * 300;
    let c = effect::get_handler(d, a.e).nil();
    for tc in group::iter(d, &g) {
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        effect::set_value(d, e1, atk);
        card::register_effect(d, tc, e1, false);
    }
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD | EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_PHASE | PHASE_END);
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(descon)));
    effect::set_operation(d, e2, Some(operation(desop)));
    duel::register_effect(d, e2, tp);
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_PLANT)), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_PLANT)), tp, LOCATION_MZONE, 0, NO_EX);
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
