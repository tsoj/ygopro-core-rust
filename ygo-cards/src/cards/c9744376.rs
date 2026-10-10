//! Good Goblin Housekeeping
use super::prelude::*;

pub const ID: u32 = 9744376;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DRAW + CATEGORY_TODECK);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, 1);
    }
    duel::set_target_player(d, tp);
    let ct = duel::get_matching_group_count(d, Some(&|d, c| card::is_code(d, c, &[ID])), tp, LOCATION_GRAVE, 0, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, (ct + 1) as i32);
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, tp, LOCATION_HAND as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    let n = duel::get_matching_group_count(d, Some(&|d, c| card::is_code(d, c, &[ID])), p, LOCATION_GRAVE, 0, NO_EX) as u32 + 1;
    duel::draw(d, p, n, REASON_EFFECT);
    duel::break_effect(d);
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    let g = duel::select_matching_card(d, p, None, p, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    duel::send_to_deck(d, g, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None);
}
