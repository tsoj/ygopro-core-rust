//! Watch Dog
use super::prelude::*;

pub const ID: u32 = 86889202;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // sset
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_operation(d, e1, Some(operation(regop)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SET);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(cond)));
    effect::set_cost(d, e2, Some(cost_target(cost_fn)));
    effect::set_target(d, e2, Some(cost_target(target)));
    effect::set_operation(d, e2, Some(operation(op)));
    card::register_effect(d, c, e2, false);
}
fn regop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, c, ID) != 0 && duel::is_phase(d, PHASE_MAIN2)
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_spell(d, c) && card::is_able_to_grave_as_cost(d, c)
}
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::get_activity_count(d, tp, &[ACTIVITY_SPSUMMON]).first().copied().unwrap_or(0) == 0
            && duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::discard_hand(d, tp, Some(&cfilter), 1, 1, REASON_COST, NO_EX);
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    effect::set_target_range(d, e1, 1, 0);
    duel::register_effect(d, e1, tp);
    let e2 = effect::create_effect(d, c);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    effect::set_target_range(d, e2, 1, 0);
    duel::register_effect(d, e2, tp);
    true
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_continuous_spell(d, c) && card::is_s_setable(d, c, false, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    let g = duel::select_matching_card(d, tp, Some(&filter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        let tc = group::get_first(d, &g).nil();
        duel::s_set(d, tp, tc, None, true);
    }
}
