//! Needlebug Nest
use super::prelude::*;

pub const ID: u32 = 84968490;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // discard deck
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_category(d, e1, CATEGORY_DECKDES);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target(d, e1, Some(cost_target(distarget)));
    effect::set_operation(d, e1, Some(operation(disop)));
    card::register_effect(d, c, e1, false);
}
fn distarget(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck(d, a.tp, 5);
    }
    duel::set_target_player(d, a.tp);
    duel::set_target_param(d, 5);
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, a.tp, 5);
    true
}
fn disop(d: &mut Duel, _a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::discard_deck(d, info[0].int() as u8, info[1].int() as u16, REASON_EFFECT);
}
