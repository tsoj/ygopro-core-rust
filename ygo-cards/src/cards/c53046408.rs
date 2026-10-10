//! Emergency Provisions
use super::prelude::*;

pub const ID: u32 = 53046408;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_RECOVER);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_spell_trap(d, c) && card::is_able_to_grave_as_cost(d, c)
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&costfilter), tp, LOCATION_ONFIELD, 0, 1, c.into());
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&costfilter), tp, LOCATION_ONFIELD, 0, 1, 5, false, c.into()).nil();
    let n = g.len() as i64;
    duel::send_to_grave(d, g, REASON_COST, None, None);
    effect::set_label(d, a.e, &[n]);
    true
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let ct = effect::get_label(d, a.e);
    duel::set_target_player(d, a.tp);
    duel::set_target_param(d, (ct * 1000) as u32);
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, (ct * 1000) as i32);
    true
}
fn activate(d: &mut Duel, _a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::recover(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
