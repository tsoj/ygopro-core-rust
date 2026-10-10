//! Coach Goblin
use super::prelude::*;

pub const ID: u32 = 42541548;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // draw
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DRAW);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(drcon)));
    effect::set_cost(d, e1, Some(cost_target(drcost)));
    effect::set_target(d, e1, Some(cost_target(drtg)));
    effect::set_operation(d, e1, Some(operation(drop_op)));
    card::register_effect(d, c, e1, false);
}
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, a.tp)
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_type(d, c, TYPE_NORMAL) && card::is_able_to_deck_as_cost(d, c)
}
fn drcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    let g = duel::select_matching_card(d, tp, Some(&cfilter), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    duel::confirm_cards(d, 1 - tp, &g);
    duel::send_to_deck(d, &g, None, SEQ_DECKBOTTOM as i32, REASON_COST, None);
    true
}
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, 1);
    }
    duel::set_target_player(d, tp);
    duel::set_target_param(d, 1);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    true
}
fn drop_op(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) || card::is_facedown(d, h) {
        return;
    }
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
