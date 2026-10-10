//! Graceful Tear
use super::prelude::*;

pub const ID: u32 = 9032529;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_RECOVER);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0) > 0;
    }
    duel::set_target_player(d, a.tp);
    duel::set_target_param(d, 2000);
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, 2000);
    true
}
fn activate(d: &mut Duel, _a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    let amount = info[1].int();
    let g = duel::get_field_group(d, p, LOCATION_HAND, 0);
    if g.is_empty() {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, p, aux::stringid(ID, 0));
    let sg = group::select(d, &g, p, 1, 1, false, NO_EX).nil();
    if duel::send_to_hand(d, sg, Some(1 - p), REASON_EFFECT, None) != 0 {
        duel::shuffle_hand(d, p);
        duel::break_effect(d);
        duel::recover(d, p, amount, REASON_EFFECT, false, None);
    }
}
