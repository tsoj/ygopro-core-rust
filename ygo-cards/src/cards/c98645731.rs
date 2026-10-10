//! Pot of Duality
use super::prelude::*;

pub const ID: u32 = 98645731;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Excavate the top 3 cards of your Deck, add 1 of them to your hand, also, after that, shuffle the rest back into your Deck
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND | CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_activity_count(d, a.tp, &[ACTIVITY_SPSUMMON])[0] == 0;
    }
    // You cannot Special Summon during the turn you activate this card
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_description(d, e1, aux::stringid(ID, 1));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET | EFFECT_FLAG_OATH | EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
    true
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let g = duel::get_decktop_group(d, a.tp, 3);
        return g.len() == 3 && group::is_exists(d, &g, &|d, c| card::is_able_to_hand(d, c, None), 1, NO_EX);
    }
    duel::set_target_player(d, a.tp);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn activate(d: &mut Duel, _a: &EffArgs) {
    let affected_player = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    let decktop3 = duel::get_decktop_group(d, affected_player, 3);
    if decktop3.is_empty() {
        return;
    }
    duel::confirm_decktop(d, affected_player, 3);
    let mut hint: u32 = HINTMSG_ATOHAND;
    let mut use_filter = true;
    if !group::is_exists(d, &decktop3, &|d, c| card::is_able_to_hand(d, c, None), 1, NO_EX) {
        hint = HINTMSG_TOGRAVE;
        use_filter = false;
    }
    // filter: Card.IsAbleToHand, or aux.TRUE (nil) when no card can be added to the hand
    let able = |d: &mut Duel, c: CardId| card::is_able_to_hand(d, c, None);
    let filter: Option<&dyn Fn(&mut Duel, CardId) -> bool> = if use_filter { Some(&able) } else { None };
    duel::hint(d, HINT_SELECTMSG as u8, affected_player, hint as u64);
    let sc = group::filter_select(d, &decktop3, affected_player, filter.unwrap_or(&|_d, _c| true), 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sc).nil();
    if card::is_able_to_hand(d, sc, None) {
        duel::send_to_hand(d, sc, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - affected_player, sc);
        duel::shuffle_hand(d, affected_player);
    } else {
        duel::send_to_grave(d, sc, REASON_RULE, None, None);
    }
    duel::break_effect(d);
    duel::shuffle_deck(d, affected_player);
}
