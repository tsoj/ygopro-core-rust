//! Card Scanner
use super::prelude::*;

pub const ID: u32 = 77066768;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Reveal the bottom card of the decks
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_TOHAND | CATEGORY_SEARCH);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_target(d, e2, Some(cost_target(decltg)));
    effect::set_operation(d, e2, Some(operation(declop)));
    card::register_effect(d, c, e2, false);
    // Make the opponent place 1 card on the bottom of the Deck
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_TODECK);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE | EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e3, EVENT_DESTROYED);
    effect::set_condition(d, e3, Some(condition(tdcond)));
    effect::set_target(d, e3, Some(cost_target(tdtg)));
    effect::set_operation(d, e3, Some(operation(tdop)));
    card::register_effect(d, c, e3, false);
}
fn decltg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, tp, LOCATION_DECK, 0) > 0 && duel::get_field_group_count(d, tp, 0, LOCATION_DECK) > 0;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_OPTION as u64);
    let op = duel::select_option(d, tp, true, &[DECLTYPE_MONSTER as u64, DECLTYPE_SPELL as u64, DECLTYPE_TRAP as u64]);
    effect::set_label(d, a.e, &[op as i64]);
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, PLAYER_ALL as u8, LOCATION_DECK as i32);
    true
}
fn declop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::get_field_group_count(d, tp, LOCATION_DECK, 0) == 0 || duel::get_field_group_count(d, tp, 0, LOCATION_DECK) == 0 {
        return;
    }
    let label = effect::get_label(d, a.e);
    let decl_type: u32 = match label {
        0 => TYPE_MONSTER,
        1 => TYPE_SPELL,
        2 => TYPE_TRAP,
        _ => 0,
    };
    let c1 = group::get_first(d, &duel::get_deckbottom_group(d, tp, 1));
    let c2 = group::get_first(d, &duel::get_deckbottom_group(d, 1 - tp, 1));
    duel::confirm_cards(d, 1 - tp, c1.nil());
    duel::confirm_cards(d, tp, c2.nil());
    duel::disable_shuffle_check(d, true);
    if let Some(c1) = c1 {
        if card::is_type(d, c1, decl_type) && card::is_able_to_hand(d, c1, None) {
            duel::send_to_hand(d, c1, None, REASON_EFFECT, None);
            duel::confirm_cards(d, 1 - tp, c1);
            duel::shuffle_hand(d, tp);
        } else {
            duel::move_sequence(d, c1, SEQ_DECKTOP as u8, None);
            duel::confirm_decktop(d, tp, 1);
        }
    }
    if let Some(c2) = c2 {
        if card::is_type(d, c2, decl_type) && card::is_able_to_hand(d, c2, None) {
            duel::send_to_hand(d, c2, None, REASON_EFFECT, None);
            duel::confirm_cards(d, tp, c2);
            duel::shuffle_hand(d, 1 - tp);
        } else {
            duel::move_sequence(d, c2, SEQ_DECKTOP as u8, None);
            duel::confirm_decktop(d, 1 - tp, 1);
        }
    }
}
fn tdcond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    a.rp == 1 - a.tp
        && card::is_reason(d, c, REASON_EFFECT)
        && card::is_previous_controler(d, c, a.tp)
        && card::is_previous_location(d, c, LOCATION_STZONE)
        && card::is_previous_position(d, c, POS_FACEUP as u32)
}
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&card::is_able_to_deck), a.tp, 0, LOCATION_HAND, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, 1 - a.tp, LOCATION_HAND as i32);
    true
}
fn tdop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    let g = duel::select_matching_card(d, 1 - a.tp, Some(&card::is_able_to_deck), 1 - a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_deck(d, g, None, SEQ_DECKBOTTOM as i32, REASON_RULE, Some(1 - a.tp));
    }
}
