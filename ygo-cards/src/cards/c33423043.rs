//! D.D. Designator
use super::prelude::*;

pub const ID: u32 = 33423043;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_category(d, e1, CATEGORY_REMOVE);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::get_field_group_count(d, tp, 0, LOCATION_HAND) > 0
            && duel::is_existing_matching_card(d, None, tp, LOCATION_HAND, 0, 1, h.into());
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CODE as u64);
    let ac = duel::announce_card(d, tp, &[TYPE_EXTRA as u64, OPCODE_ISTYPE as u64, OPCODE_NOT as u64]);
    duel::set_target_param(d, ac as u32);
    duel::set_operation_info(d, 0, CATEGORY_ANNOUNCE, None, 0, tp, ANNOUNCE_CARD_FILTER as i32);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let ac = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int() as u32;
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_code(d, c, &[ac])), tp, 0, LOCATION_HAND, NO_EX);
    let hg = duel::get_field_group(d, tp, 0, LOCATION_HAND);
    duel::confirm_cards(d, tp, &hg);
    if !g.is_empty() {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        duel::remove(d, sg, POS_FACEUP as u8, REASON_EFFECT, None, None);
        duel::shuffle_hand(d, 1 - tp);
    } else {
        let sg = duel::get_field_group(d, tp, LOCATION_HAND, 0);
        let dg = group::random_select(d, &sg, tp, 1);
        duel::remove(d, dg, POS_FACEUP as u8, REASON_EFFECT, None, None);
        duel::shuffle_hand(d, 1 - tp);
    }
}
