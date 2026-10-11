//! SPYRAL Master Plan
use super::prelude::*;

pub const ID: u32 = 91258852;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SPYRAL_MISSION, SET_SPYRAL], listed_names: &[54631665], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 1 "SPYRAL MISSION" card from your Deck to your hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_target(d, e1, Some(cost_target(thtg1)));
    effect::set_operation(d, e1, Some(operation(thop1)));
    card::register_effect(d, c, e1, false);
    // Add 1 "SPYRAL Resort" and 1 "SPYRAL" monster from your Deck to your hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(thcon2)));
    effect::set_target(d, e2, Some(cost_target(thtg2)));
    effect::set_operation(d, e2, Some(operation(thop2)));
    card::register_effect(d, c, e2, false);
}
/// `s.thfilter1(c)`
fn thfilter1(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_SPYRAL_MISSION as u16]) && card::is_able_to_hand(d, c, None)
}
fn thtg1(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter1), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn thop1(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&thfilter1), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, g);
    }
}
fn thcon2(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, h, LOCATION_ONFIELD)
}
/// `s.thfilter2(c,tp)`
fn thfilter2(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_code(d, c, &[54631665])
        && card::is_able_to_hand(d, c, None)
        && duel::is_existing_matching_card(d, Some(&thfilter3), tp, LOCATION_DECK, 0, 1, c.into())
}
/// `s.thfilter3(c)`
fn thfilter3(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_SPYRAL as u16]) && card::is_monster(d, c) && !card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}
fn thtg2(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| thfilter2(d, c, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
fn thop2(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g1 = duel::select_matching_card(d, tp, Some(&|d, c| thfilter2(d, c, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g1.is_empty() {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        let first = group::get_first(d, &g1).nil();
        let g2 = duel::select_matching_card(d, tp, Some(&thfilter3), tp, LOCATION_DECK, 0, 1, 1, false, first.into()).nil();
        let mut g1 = g1;
        group::add_card(&mut g1, &g2);
        duel::send_to_hand(d, g1.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, g1);
    }
}
