//! Dinomist Charge
use super::prelude::*;

pub const ID: u32 = 77116346;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DINOMIST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // tohand
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_TRIGGER_F + EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EVENT_TO_DECK);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(thcon)));
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_DINOMIST as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if !card::is_relate_to_effect(d, effect::get_handler(d, a.e).nil(), a.e) {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&filter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, &g);
    }
}
fn thfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_faceup(d, c)
        && card::is_controler(d, c, tp)
        && card::is_location(d, c, LOCATION_EXTRA)
        && card::is_set_card(d, c, &[SET_DINOMIST as u16])
        && card::is_previous_location(d, c, LOCATION_ONFIELD)
}
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| thfilter(d, c, tp), 1, NO_EX)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| thfilter(d, c, tp), NO_EX);
    duel::set_target_card(d, &g);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g.clone()), 1, 0, 0);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if !card::is_relate_to_effect(d, effect::get_handler(d, a.e).nil(), a.e) {
        return;
    }
    let g = duel::get_target_cards(d, a.e).nil();
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let rg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
    if !rg.is_empty() {
        duel::send_to_hand(d, &rg, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, &rg);
    }
}
