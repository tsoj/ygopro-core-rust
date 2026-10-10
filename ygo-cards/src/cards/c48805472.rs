//! Traptantalizing Tune
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 48805472;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Draw 2 cards
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DRAW);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(drcost)));
    effect::set_target(d, e1, Some(cost_target(drtg)));
    effect::set_operation(d, e1, Some(operation(drop_op)));
    card::register_effect(d, c, e1, false);
    // Place card on the bottom of the Deck
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TODECK);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    effect::set_operation(d, e2, Some(operation(tdop)));
    card::register_effect(d, c, e2, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    (card::is_monster(d, c) && card::is_level(d, c, &[4]) && card::is_race(d, c, RACE_INSECT | RACE_PLANT)) || card::is_normal_trap(d, c)
}
/// `s.drcostfilter = aux.AND(s.filter, Card.IsDiscardable)`
fn drcostfilter(d: &mut Duel, c: CardId) -> bool {
    filter(d, c) && card::is_discardable(d, c, None, None)
}
/// `s.drcost(e,tp,...,chk)`
fn drcost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&drcostfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::discard_hand(d, a.tp, Some(&drcostfilter), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
/// `s.drtg(e,tp,...,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 2);
    }
    duel::set_target_player(d, a.tp);
    duel::set_target_param(d, 2);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 2);
    true
}
/// `s.drop(e,...)`
fn drop_op(d: &mut Duel, _a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
/// `s.tdfilter = aux.AND(Card.IsFaceup, s.filter, Card.IsAbleToDeck)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && filter(d, c) && card::is_able_to_deck(d, c)
}
/// `s.tdtg(e,tp,...,chk,chkc)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_REMOVED) && card::is_controler(d, chkc, a.tp) && tdfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tdfilter), a.tp, LOCATION_REMOVED, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    let g = duel::select_target(d, a.tp, Some(&tdfilter), a.tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_TODECK, g, 1, 0, 0);
    true
}
/// `s.tdop(e,tp,...)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::send_to_deck(d, tc, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None);
    }
}
