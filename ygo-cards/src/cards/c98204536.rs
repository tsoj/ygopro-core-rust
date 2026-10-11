//! Chronomaly Temple - Trilithon
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 98204536;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CHRONOMALY, SET_NUMBER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e0, false);
    // Normal Summon
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_SZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(nscon)));
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(500))));
    effect::set_target(d, e1, Some(cost_target(nstg)));
    effect::set_operation(d, e1, Some(operation(nsop)));
    card::register_effect(d, c, e1, false);
    // Send to GY instead of detaching materials
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_OVERLAY_REMOVE_REPLACE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_condition(d, e2, Some(condition(rcon)));
    effect::set_operation(d, e2, Some(operation_count(rop)));
    card::register_effect(d, c, e2, false);
}
fn nscon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let ct = duel::get_matching_group_count(d, Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_CHRONOMALY as u16]))), tp, LOCATION_MZONE, 0, NO_EX);
    ct as i32 == duel::get_field_group_count(d, tp, LOCATION_MZONE, 0)
}
fn nsfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_CHRONOMALY as u16]) && card::is_summonable(d, c, true, None, 0, None)
}
fn nstg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&nsfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SUMMON, None, 1, 0, 0);
    true
}
fn nsop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&nsfilter), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        let first = group::get_first(d, &g).nil();
        duel::summon(d, tp, first, true, None, 0, None);
    }
}
fn rcon(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    let h = effect::get_handler(d, a.e).nil();
    (a.r & REASON_COST) != 0
        && effect::is_activated(d, re)
        && effect::is_active_type(d, re, TYPE_XYZ)
        && (card::is_set_card(d, rc, &[SET_CHRONOMALY as u16]) || card::is_set_card(d, rc, &[SET_NUMBER as u16]))
        && card::is_able_to_grave_as_cost(d, h)
        && a.ep == effect::get_owner_player(d, a.e)
        && a.ev >= 1
        && card::get_overlay_count(d, rc) as i64 >= a.ev as i64 - 1
}
/// `return Duel.SendtoGrave(e:GetHandler(), REASON_COST)`: a number is always true in Lua.
fn rop(d: &mut Duel, a: &EffArgs) -> i64 {
    let h = effect::get_handler(d, a.e).nil();
    duel::send_to_grave(d, h, REASON_COST, None, None) as i64
}
