//! Mind Crush
use super::prelude::*;

pub const ID: u32 = 15800838;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_TOHAND));
    effect::set_category(d, e1, CATEGORY_HANDES);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, tp, 0, LOCATION_HAND) > 0
            && duel::is_existing_matching_card(d, None, tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CODE as u64);
    // s.announce_filter={TYPE_EXTRA,OPCODE_ISTYPE,TYPE_MONSTER,OPCODE_ISTYPE,OPCODE_AND,OPCODE_NOT}
    let ac = duel::announce_card(d, tp, &[TYPE_EXTRA as u64, OPCODE_ISTYPE, TYPE_MONSTER as u64, OPCODE_ISTYPE, OPCODE_AND, OPCODE_NOT]);
    duel::set_target_param(d, ac as u32);
    duel::set_operation_info(d, 0, CATEGORY_ANNOUNCE, None, 0, tp, ANNOUNCE_CARD_FILTER as i32);
    true
}
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ac=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let ac = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int() as u32;
    // local g=Duel.GetMatchingGroup(Card.IsCode,tp,0,LOCATION_HAND,nil,ac)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_code(d, c, &[ac])), tp, 0, LOCATION_HAND, NO_EX);
    if !g.is_empty() {
        duel::send_to_grave(d, g, REASON_EFFECT | REASON_DISCARD, None, None);
    } else {
        let sg = duel::get_field_group(d, tp, LOCATION_HAND, 0);
        let dg = group::random_select(d, &sg, tp, 1);
        duel::send_to_grave(d, dg, REASON_EFFECT | REASON_DISCARD, None, None);
    }
}
