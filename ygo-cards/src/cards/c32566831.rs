//! Cards of the Red Stone
use super::prelude::*;

pub const ID: u32 = 32566831;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_RED_EYES], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DRAW);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_RED_EYES as u16]) && card::get_level(d, c) == 7 && card::is_able_to_grave_as_cost(d, c)
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::discard_hand(d, a.tp, Some(&cfilter), 1, 1, REASON_COST, NO_EX);
    true
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, 2);
    }
    duel::set_target_player(d, tp);
    duel::set_target_param(d, 2);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 2);
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
    true
}
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_RED_EYES as u16]) && card::get_level(d, c) == 7 && card::is_able_to_grave(d, c, None)
}
fn activate(d: &mut Duel, _a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    let dcount = info[1].int() as u32;
    let dr = duel::draw(d, p, dcount, REASON_EFFECT);
    let g = duel::get_matching_group(d, Some(&tgfilter), p, LOCATION_DECK, 0, NO_EX);
    if dr != 0 && !g.is_empty() && duel::select_yes_no(d, p, aux::stringid(ID, 0)) {
        duel::break_effect(d);
        duel::hint(d, HINT_SELECTMSG as u8, p, HINTMSG_TOGRAVE as u64);
        let sg = group::select(d, &g, p, 1, 1, false, NO_EX).nil();
        duel::send_to_grave(d, sg, REASON_EFFECT, None, None);
    }
}
