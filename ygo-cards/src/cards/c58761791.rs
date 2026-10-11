//! Shipping Error
use super::prelude::*;

pub const ID: u32 = 58761791;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // An activated effect becomes "Your opponent takes 1 card from their Deck for you to add to your hand, and you must keep that card revealed"
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_condition(d, e1, Some(condition(changecon)));
    effect::set_target(d, e1, Some(cost_target(changetg)));
    effect::set_operation(d, e1, Some(operation(changeop)));
    card::register_effect(d, c, e1, false);
}
fn changecon(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    if effect::is_has_category(d, re, CATEGORY_SEARCH) || effect::is_has_category(d, re, CATEGORY_DRAW) {
        return true;
    }
    let info1 = duel::get_operation_info(d, a.ev, CATEGORY_TOHAND);
    let info2 = duel::get_possible_operation_info(d, a.ev, CATEGORY_TOHAND);
    let mut g = CardSet::new();
    let loc1 = info1.as_ref().map_or(0, |i| i.op_param as i64);
    let loc2 = info2.as_ref().map_or(0, |i| i.op_param as i64);
    if let Some(cards) = info1.as_ref().and_then(|i| i.op_cards.clone()) {
        group::add_card(&mut g, &cards);
    }
    if let Some(cards) = info2.as_ref().and_then(|i| i.op_cards.clone()) {
        group::add_card(&mut g, &cards);
    }
    ((loc1 | loc2) & LOCATION_DECK as i64) > 0 || (!g.is_empty() && group::is_exists(d, &g, &|d, c| card::is_location(d, c, LOCATION_DECK), 1, NO_EX))
}
fn changetg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), 1 - a.rp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
fn changeop(d: &mut Duel, a: &EffArgs) {
    let g = CardSet::new();
    duel::change_target_card(d, a.ev as u8, &g);
    duel::change_chain_operation(d, a.ev as u8, operation(replaceop));
}
fn replaceop(d: &mut Duel, a: &EffArgs) {
    let opp = 1 - a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, opp, aux::stringid(ID, 1));
    let g = duel::select_matching_card(d, opp, Some(&|d, c| card::is_able_to_hand(d, c, None)), opp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    if let Some(sc) = sc {
        if duel::send_to_hand(d, sc, Some(a.tp), REASON_EFFECT, None) > 0 {
            let c = effect::get_handler(d, a.e).nil();
            // Keep it revealed
            let e1 = effect::create_effect(d, c);
            effect::set_description(d, e1, aux::stringid(ID, 2));
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
            effect::set_code(d, e1, EFFECT_PUBLIC);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, sc, e1, false);
            // During the End Phase of this turn, shuffle it into the Deck, and if you do, draw 1 card
            let tp = a.tp;
            aux::delayed_operation(
                d,
                sc,
                PHASE_END,
                ID,
                a.e,
                tp,
                Some(std::sync::Arc::new(move |d: &mut Duel, _ag: &CardSet, _a: &EffArgs| {
                    if duel::send_to_deck(d, sc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0 && card::is_location(d, sc, LOCATION_DECK) {
                        duel::draw(d, tp, 1, REASON_EFFECT);
                    }
                    true
                })),
                None,
                None,
                None,
                None,
                None,
            );
        }
    }
}
