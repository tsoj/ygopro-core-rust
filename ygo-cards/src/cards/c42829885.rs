//! The Forceful Sentry
use super::prelude::*;

pub const ID: u32 = 42829885;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_TODECK);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND) > 0;
    }
    duel::set_target_player(d, a.tp);
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 0, 1 - a.tp, LOCATION_HAND as i32);
    true
}
fn activate(d: &mut Duel, _a: &EffArgs) {
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    let g = duel::get_field_group(d, p, 0, LOCATION_HAND);
    if !g.is_empty() {
        duel::confirm_cards(d, p, &g);
        duel::hint(d, HINT_SELECTMSG as u8, p, HINTMSG_TODECK as u64);
        let sg = group::select(d, &g, p, 1, 1, false, NO_EX).nil();
        duel::send_to_deck(d, sg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        duel::shuffle_hand(d, 1 - p);
    }
}
