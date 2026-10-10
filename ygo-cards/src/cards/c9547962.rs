//! Euler's Circuit
use super::prelude::*;

pub const ID: u32 = 9547962;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_TINDANGLE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // cannot attack
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_CANNOT_ATTACK);
    effect::set_range(d, e2, LOCATION_FZONE);
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    effect::set_condition(d, e2, Some(condition(atkcon)));
    card::register_effect(d, c, e2, false);
    // give control
    let e3 = effect::create_effect(d, c);
    effect::set_category(d, e3, CATEGORY_CONTROL);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e3, EVENT_PHASE | PHASE_STANDBY);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e3, LOCATION_FZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_condition(d, e3, Some(condition(ctcon)));
    effect::set_target(d, e3, Some(cost_target(cttg)));
    effect::set_operation(d, e3, Some(operation(ctop)));
    card::register_effect(d, c, e3, false);
    // search
    let e5 = effect::create_effect(d, c);
    effect::set_category(d, e5, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e5, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e5, LOCATION_GRAVE);
    effect::set_count_limit(d, e5, 1, ID, 0, 0);
    effect::set_cost(d, e5, Some(cost_target(thcost)));
    effect::set_target(d, e5, Some(cost_target(thtg)));
    effect::set_operation(d, e5, Some(operation(thop)));
    card::register_effect(d, c, e5, false);
}
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let p = effect::get_handler_player(d, a.e);
    let f = aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_TINDANGLE as u16]));
    duel::is_existing_matching_card(d, Some(&f), p, LOCATION_MZONE, 0, 3, NO_EX)
}
fn ctcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, a.tp)
}
fn ctfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_TINDANGLE as u16]) && card::is_controler_can_be_changed(d, c, false, None)
}
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && ctfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&ctfilter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONTROL as u64);
    let g = duel::select_target(d, tp, Some(&ctfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(g), 1, 0, 0);
    true
}
fn ctop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    let tc = duel::first_target(d);
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            duel::get_control(d, tc, 1 - a.tp, 0, 0, None, None);
        }
    }
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_TINDANGLE as u16]) && card::is_discardable(d, c, None, None)
}
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_remove_as_cost(d, h, None) && duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::remove(d, h, POS_FACEUP as u8, REASON_COST, None, None);
    duel::discard_hand(d, tp, Some(&cfilter), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::get_first_matching_card(d, Some(&filter), a.tp, LOCATION_DECK, 0, NO_EX);
    if let Some(tc) = tc {
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
    }
}
