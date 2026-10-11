//! Sandwich Day
use super::prelude::*;

pub const ID: u32 = 83838727;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Draw 1 card and show it
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DRAW + CATEGORY_HANDES);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(200))));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&card::is_monster), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1);
    }
    duel::set_target_player(d, a.tp);
    duel::set_target_param(d, 1);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    duel::set_possible_operation_info(d, 0, CATEGORY_HANDES, None, 0, a.tp, 1);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    let cnt = info[1].int() as u32;
    if duel::draw(d, p, cnt, REASON_EFFECT) == 0 {
        return;
    }
    let og = duel::get_operated_group(d);
    let dc = group::get_first(d, &og).nil();
    duel::confirm_cards(d, 1 - a.tp, CardSet::from_card(dc));
    duel::shuffle_hand(d, a.tp);
    if !card::is_monster(d, dc) {
        return;
    }
    duel::break_effect(d);
    let att = card::get_attribute(d, dc);
    if !duel::is_existing_matching_card(d, Some(&|d, c| card::is_attribute(d, c, att)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX) {
        // Draw 1 card
        duel::draw(d, a.tp, 1, REASON_EFFECT);
    } else {
        // Discard 1 card
        duel::discard_hand(d, a.tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
    }
}
