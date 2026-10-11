//! Garden of Talistones
use super::prelude::*;

pub const ID: u32 = 10307853;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e0, false);
    // While you have 5 cards in your Spell & Trap Zone, any battle damage a player takes is halved, also once per turn, you can activate 1 Continuous Trap Card the turn it was Set
    let e1a = effect::create_effect(d, c);
    effect::set_type(d, e1a, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1a, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1a, EFFECT_CHANGE_BATTLE_DAMAGE);
    effect::set_range(d, e1a, LOCATION_SZONE);
    effect::set_target_range(d, e1a, 1, 1);
    effect::set_condition(d, e1a, Some(condition(e1a_cond)));
    effect::set_value(d, e1a, HALF_DAMAGE as i64);
    card::register_effect(d, c, e1a, false);
    let e1b = effect::clone(d, e1a);
    effect::set_description(d, e1b, aux::stringid(ID, 0));
    effect::set_property(d, e1b, EFFECT_FLAG_SET_AVAILABLE, 0);
    effect::set_code(d, e1b, EFFECT_TRAP_ACT_IN_SET_TURN);
    effect::set_target_range(d, e1b, LOCATION_SZONE, 0);
    effect::set_count_limit(d, e1b, 1, 0, 0, 0);
    card::register_effect(d, c, e1b, false);
    // During the Standby Phase: You can reveal 5 Continuous Traps with different names from your Deck, your opponent randomly picks 1 for you to Set on your field, also shuffle the rest into the Deck. You can only use this effect of "Protective Stone Gardenscaping" once per turn
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SET);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    effect::set_target(d, e2, Some(cost_target(settg)));
    effect::set_operation(d, e2, Some(operation(setop)));
    effect::set_hint_timing(d, e2, TIMING_STANDBY_PHASE, None);
    card::register_effect(d, c, e2, false);
}
fn e1a_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_field_group_count(d, effect::get_handler_player(d, a.e), LOCATION_STZONE, 0) == 5
}
fn e2_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_standby_phase(d, None)
}
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_continuous_trap(d, c) && card::is_s_setable(d, c, false, None) && !card::is_public(d, c)
}
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    let g = duel::get_matching_group(d, Some(&setfilter), a.tp, LOCATION_DECK, 0, NO_EX);
    if a.chk == Some(0) {
        return g.len() >= 5 && group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) >= 5;
    }
    duel::set_operation_info(d, 0, CATEGORY_SET, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn setop(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_matching_group(d, Some(&setfilter), a.tp, LOCATION_DECK, 0, NO_EX);
    if g.len() < 5 {
        return;
    }
    let rg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(5), Some(5), Some(&aux::dncheck()), 1, a.tp, Some(HINTMSG_CONFIRM as u64), None, None, false).unwrap_or_default();
    duel::confirm_cards(d, 1 - a.tp, rg.clone());
    duel::shuffle_deck(d, a.tp);
    let sg = group::random_select(d, &rg, 1 - a.tp, 1);
    if !sg.is_empty() {
        duel::disable_shuffle_check(d, true);
        duel::s_set(d, a.tp, sg, Some(a.tp), false);
    }
}
