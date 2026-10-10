//! Treasure Map
use super::prelude::*;

pub const ID: u32 = 54762426;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DRAW + CATEGORY_HANDES);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let handler = effect::get_handler(d, a.e).nil();
    match duel::get_operation_info(d, a.ev, CATEGORY_TOHAND) {
        None => false,
        Some(info) => match info.op_cards {
            Some(tg) => group::is_contains(&tg, handler),
            None => false,
        },
    }
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 2);
    }
    duel::set_target_player(d, a.tp);
    duel::set_target_param(d, 2);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 2);
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, a.tp, 1);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    let count = info[1].int() as u32;
    if duel::draw(d, p, count, REASON_EFFECT) == 2 {
        duel::break_effect(d);
        duel::discard_hand(d, a.tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
    }
}
