//! The Inexperienced Spy
use super::prelude::*;

pub const ID: u32 = 81820689;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // confirm
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(cftg)));
    effect::set_operation(d, e1, Some(operation(cfop)));
    card::register_effect(d, c, e1, false);
}
fn cftg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND) > 0;
    }
    duel::set_target_player(d, a.tp);
    true
}
fn cfop(d: &mut Duel, _a: &EffArgs) {
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    duel::hint(d, HINT_SELECTMSG as u8, p, HINTMSG_CONFIRM as u64);
    let g = duel::select_matching_card(d, p, None, p, 0, LOCATION_HAND, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::confirm_cards(d, p, g);
        duel::shuffle_hand(d, 1 - p);
    }
}
