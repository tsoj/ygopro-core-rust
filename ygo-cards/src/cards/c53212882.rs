//! Floowandereeze & Snowl
use super::prelude::*;

pub const ID: u32 = 53212882;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Can conduct 3 normal summons
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(trcon)));
    effect::set_target(d, e1, Some(cost_target(nstg)));
    effect::set_operation(d, e1, Some(operation(nsop)));
    card::register_effect(d, c, e1, false);
    // Piercing damage
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_code(d, e2, EFFECT_PIERCE);
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    effect::set_condition(d, e2, Some(condition(trcon)));
    card::register_effect(d, c, e2, false);
    // Flip opponent monsters face-down
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_POSITION | CATEGORY_SET);
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_condition(d, e3, Some(condition(fdcon)));
    effect::set_cost(d, e3, Some(cost_target(fdcost)));
    effect::set_target(d, e3, Some(cost_target(fdtg)));
    effect::set_operation(d, e3, Some(operation(fdop)));
    effect::set_hint_timing(d, e3, 0, Some(TIMING_END_PHASE));
    card::register_effect(d, c, e3, false);
}
fn trcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_tribute_summoned(d, h)
}
fn nstg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let mut ct: i64 = 0;
        if let Some(te) = duel::is_player_affected_by_effect(d, a.tp, EFFECT_SET_SUMMON_COUNT_LIMIT) {
            ct = ct.max(effect::get_value(d, te).int());
        }
        return ct < 3;
    }
    true
}
fn nsop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_SET_SUMMON_COUNT_LIMIT);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_value(d, e1, 3);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
}
fn fdcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp)
}
fn fdcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_remove_as_cost(d, c, None)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_able_to_remove_as_cost(d, c, None)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
fn fdfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_can_turn_set(d, c) && card::is_special_summoned(d, c)
}
fn fdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let g = duel::get_matching_group(d, Some(&fdfilter), a.tp, 0, LOCATION_MZONE, NO_EX);
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(g), n, a.tp, POS_FACEDOWN_DEFENSE as i32);
    true
}
fn fdop(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_matching_group(d, Some(&fdfilter), a.tp, 0, LOCATION_MZONE, NO_EX);
    if !g.is_empty() {
        duel::change_position(d, g, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
    }
}
