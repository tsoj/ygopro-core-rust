//! Card of Fate
use super::prelude::*;

pub const ID: u32 = 74191528;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(thtg)));
    effect::set_operation(d, e1, Some(operation(thop)));
    card::register_effect(d, c, e1, false);
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 5, NO_EX)
            && duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 5, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 0, PLAYER_ALL as u8, 1);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if !duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 5, NO_EX)
        && duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 5, NO_EX)
    {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g1 = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_ATOHAND as u64);
    let g2 = duel::select_matching_card(d, 1 - tp, Some(&thfilter), 1 - tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    duel::shuffle_deck(d, 1 - tp);
    if !g1.is_empty() && !g2.is_empty() {
        duel::break_effect(d);
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        let g3 = duel::select_matching_card(d, tp, Some(&thfilter), tp, 0, LOCATION_DECK, 4, 4, false, (&g2).into()).nil();
        duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_ATOHAND as u64);
        let g4 = duel::select_matching_card(d, 1 - tp, Some(&thfilter), 1 - tp, 0, LOCATION_DECK, 4, 4, false, (&g1).into()).nil();
        if g3.len() == 4 && g4.len() == 4 {
            let mut ga = g1.clone();
            group::add_card(&mut ga, &g4);
            let mut gb = g2.clone();
            group::add_card(&mut gb, &g3);
            let ca = group::random_select(d, &ga, tp, 1);
            duel::confirm_cards(d, 1 - tp, ca.clone());
            duel::send_to_hand(d, ca, Some(tp), REASON_EFFECT, None);
            let cb = group::random_select(d, &gb, 1 - tp, 1);
            duel::confirm_cards(d, tp, cb.clone());
            duel::send_to_hand(d, cb, Some(1 - tp), REASON_EFFECT, None);
            duel::shuffle_deck(d, tp);
            duel::shuffle_deck(d, 1 - tp);
        }
    }
}
