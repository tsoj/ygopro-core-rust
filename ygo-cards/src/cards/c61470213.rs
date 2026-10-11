//! Altergeist Adminia
use super::prelude::*;

pub const ID: u32 = 61470213;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ALTERGEIST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Link Summon Procedure
    link::add_procedure(d, c, Some(mat_filter(|d, x, sc, st, tp| card::is_set_card_ctx(d, x, &[SET_ALTERGEIST as u16], sc, st, Some(tp)))), 2, None, None, None, None);
    // Set 1 "Altergeist" Trap from the Deck
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SET);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(e1_cond)));
    effect::set_target(d, e1, Some(cost_target(settg)));
    effect::set_operation(d, e1, Some(operation(setop)));
    card::register_effect(d, c, e1, false);
    // Take control of an opponent's monster
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_CONTROL);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_NO_TURN_RESET, 0);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_MAIN_END));
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    effect::set_cost(d, e2, Some(cost_target(ctcost)));
    effect::set_target(d, e2, Some(cost_target(cttg)));
    effect::set_operation(d, e2, Some(operation(ctop)));
    card::register_effect(d, c, e2, false);
}
fn e1_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_link_summoned(d, h)
}
fn e2_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_main_phase(d, None)
}
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_ALTERGEIST as u16]) && card::is_type(d, c, TYPE_TRAP) && card::is_s_setable(d, c, false, None)
}
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&setfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
fn setop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&setfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::s_set(d, a.tp, g, None, true);
    }
}
fn cstfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_able_to_grave_as_cost(d, c) && duel::get_m_zone_count(d, card::get_controler(d, c), c.into(), None, None, None).0 > 0
}
fn ctcost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cstfilter), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&cstfilter), a.tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
fn ctfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_EFFECT) && card::is_able_to_change_controler(d, c)
}
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && ctfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&ctfilter), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONTROL as u64);
    let g = duel::select_target(d, tp, Some(&ctfilter), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(g), 1, tp, 0);
    true
}
fn ctop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) && card::is_type(d, tc, TYPE_EFFECT) {
        // Duel.GetControl returns a number, which is always true in Lua
        duel::get_control(d, tc, a.tp, 0, 0, None, None);
        // Monster is also treated as an "Altergeist" monster
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_ADD_SETCODE);
        effect::set_value(d, e1, SET_ALTERGEIST as i64);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_CONTROL, 1);
        card::register_effect(d, tc, e1, false);
    }
}
