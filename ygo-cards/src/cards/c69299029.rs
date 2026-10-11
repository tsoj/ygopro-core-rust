//! Treasures of the Kings
use super::prelude::*;

pub const ID: u32 = 69299029;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_APOPHIS], listed_names: &[CARD_TEMPLE_OF_THE_KINGS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When this card is activated: You can Set 1 "Apophis" Trap from your Deck
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SET);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // This card's name becomes "Temple of the Kings" while in the Field Zone
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e2, EFFECT_CHANGE_CODE);
    effect::set_range(d, e2, LOCATION_FZONE);
    effect::set_value(d, e2, CARD_TEMPLE_OF_THE_KINGS as i64);
    card::register_effect(d, c, e2, false);
    // Add 1 monster that mentions "Temple of the Kings" from your Deck to your hand
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e3, LOCATION_FZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_condition(d, e3, Some(condition(thcon)));
    effect::set_target(d, e3, Some(cost_target(thtg)));
    effect::set_operation(d, e3, Some(operation(thop)));
    card::register_effect(d, c, e3, false);
}
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_APOPHIS as u16]) && card::is_trap(d, c) && card::is_s_setable(d, c, false, None)
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let g = duel::get_matching_group(d, Some(&setfilter), tp, LOCATION_DECK, 0, NO_EX);
    if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        duel::s_set(d, tp, &sg, None, true);
    }
}
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_facedown(d, c)), tp, LOCATION_ONFIELD, 0, 2, NO_EX)
        || duel::is_existing_matching_card(d, Some(&|d, c| card::is_trap(d, c)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_monster(d, c) && card::lists_code(d, c, &[CARD_TEMPLE_OF_THE_KINGS]) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, &g);
    }
}
