//! Sylvan Bladefender
use super::prelude::*;

pub const ID: u32 = 11682713;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Excavate the top cardof your deck
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DECKDES);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYING);
    effect::set_condition(d, e1, Some(aux::bdogcon()));
    effect::set_target(d, e1, Some(cost_target(dd_target)));
    effect::set_operation(d, e1, Some(operation(dd_op)));
    card::register_effect(d, c, e1, false);
    // Add this card to the hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_condition(d, e2, Some(condition(tdcon)));
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    effect::set_operation(d, e2, Some(operation(tdop)));
    card::register_effect(d, c, e2, false);
}
fn dd_target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck(d, a.tp, 1);
    }
    true
}
fn dd_op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if !duel::is_player_can_discard_deck(d, tp, 1) {
        return;
    }
    duel::confirm_decktop(d, tp, 1);
    let g = duel::get_decktop_group(d, tp, 1);
    let tc = group::get_first(d, &g).nil();
    if card::is_race(d, tc, RACE_PLANT) {
        duel::disable_shuffle_check(d, true);
        duel::send_to_grave(d, g, REASON_EFFECT | REASON_EXCAVATE, None, None);
    } else {
        duel::move_sequence(d, tc, 1, None);
    }
}
fn tdcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, c, LOCATION_DECK) && card::is_reason(d, c, REASON_EXCAVATE)
}
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn tdop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::send_to_hand(d, c, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - a.tp, c);
    }
}
