//! Hecahands Bait
use super::prelude::*;

pub const ID: u32 = 43932352;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_HECAHANDS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e0, false);
    // You can Tribute 1 monster you control that is a "Hecahands" monster or a monster owned by your opponent; take control of 1 monster your opponent controls
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_CONTROL);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_range(d, e1, LOCATION_SZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(ctrlcost)));
    effect::set_target(d, e1, Some(cost_target(ctrltg)));
    effect::set_operation(d, e1, Some(operation(ctrlop)));
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    card::register_effect(d, c, e1, false);
    // Once per turn, during your End Phase: Send 1 card you control to the GY
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD | EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_PHASE | PHASE_END);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(|d, a| duel::is_turn_player(d, a.tp))));
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    effect::set_operation(d, e2, Some(operation(tgop)));
    card::register_effect(d, c, e2, false);
}
fn ctrlcostfilter(d: &mut Duel, c: CardId, tp: u8, opp: u8) -> bool {
    (card::is_set_card(d, c, &[SET_HECAHANDS as u16]) || card::is_owner(d, c, opp))
        && duel::get_m_zone_count(d, tp, c.into(), Some(tp), Some(LOCATION_REASON_CONTROL as u32), None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, x| card::is_able_to_change_controler(d, x)), tp, 0, LOCATION_MZONE, 1, c.into())
}
fn ctrlcost(d: &mut Duel, a: &EffArgs) -> bool {
    let opp = 1 - a.tp;
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&|d, c| ctrlcostfilter(d, c, a.tp, opp)), 1, 1, false, None, NO_EX);
    }
    let g = duel::select_release_group_cost(d, a.tp, Some(&|d, c| ctrlcostfilter(d, c, a.tp, opp)), 1, 1, false, None, NO_EX);
    duel::release(d, g, REASON_COST, None);
    true
}
fn ctrlchange_filter(d: &mut Duel, c: CardId) -> bool {
    card::is_able_to_change_controler(d, c)
}
fn ctrltg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&ctrlchange_filter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, None, 1, 1 - a.tp, LOCATION_MZONE as i32);
    true
}
fn ctrlop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONTROL as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&ctrlchange_filter), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::hint_selection(d, g.clone(), true);
        duel::get_control(d, g, a.tp, 0, 0, None, None);
    }
}
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_ONFIELD as i32);
    true
}
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_able_to_grave(d, c, None)
}
fn tgop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&tgfilter), a.tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::hint_selection(d, g.clone(), true);
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
