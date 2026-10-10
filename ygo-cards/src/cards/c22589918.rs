//! Reload
use super::prelude::*;

pub const ID: u32 = 22589918;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_TODECK | CATEGORY_DRAW);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        if !duel::is_player_can_draw(d, a.tp, 1) {
            return false;
        }
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&card::is_able_to_deck), a.tp, LOCATION_HAND, 0, 1, h.into());
    }
    duel::set_target_player(d, a.tp);
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_HAND as i32);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
fn activate(d: &mut Duel, _a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER]);
    let p = info[0].int() as u8;
    let g = duel::get_field_group(d, p, LOCATION_HAND, 0);
    if g.is_empty() {
        return;
    }
    let n = g.len() as u32;
    duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    duel::shuffle_deck(d, p);
    duel::break_effect(d);
    duel::draw(d, p, n, REASON_EFFECT);
}
