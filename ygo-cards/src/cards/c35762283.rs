//! Heart of the Underdog
use super::prelude::*;

pub const ID: u32 = 35762283;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // draw
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_DRAW);
    effect::set_condition(d, e2, Some(condition(drcon)));
    effect::set_cost(d, e2, Some(cost_target(drcost)));
    effect::set_target(d, e2, Some(cost_target(drtg)));
    effect::set_operation(d, e2, Some(operation(drop)));
    card::register_effect(d, c, e2, false);
}
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, a.tp) && duel::is_phase(d, PHASE_DRAW)
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_type(d, c, TYPE_NORMAL) && !card::is_public(d, c)
}
fn drcost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return a.ep == a.tp && group::is_exists(d, &a.eg.clone().nil(), &filter, 1, NO_EX);
    }
    let g = group::filter(d, &a.eg.clone().nil(), &filter, NO_EX);
    if g.len() == 1 {
        duel::confirm_cards(d, 1 - a.tp, g);
        duel::shuffle_hand(d, a.tp);
    } else {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONFIRM as u64);
        let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        duel::confirm_cards(d, 1 - a.tp, sg);
        duel::shuffle_hand(d, a.tp);
    }
    true
}
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1);
    }
    duel::set_target_player(d, a.tp);
    duel::set_target_param(d, 1);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
fn drop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
