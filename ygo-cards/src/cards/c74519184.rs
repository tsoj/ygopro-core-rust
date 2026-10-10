//! Hand Destruction
use super::prelude::*;

pub const ID: u32 = 74519184;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DRAW | CATEGORY_HANDES);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let mut h1 = duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0);
        let h = effect::get_handler(d, a.e).nil();
        if card::is_location(d, h, LOCATION_HAND) {
            h1 -= 1;
        }
        let h2 = duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND);
        return h1 > 1 && h2 > 1 && duel::is_player_can_draw(d, a.tp, 2) && duel::is_player_can_draw(d, 1 - a.tp, 2);
    }
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, PLAYER_ALL as u8, 2);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, PLAYER_ALL as u8, 2);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let h1 = duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0);
    let h2 = duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND);
    if h1 < 2 || h2 < 2 {
        return;
    }
    let turnp = duel::get_turn_player(d);
    duel::hint(d, HINT_SELECTMSG as u8, turnp, HINTMSG_TOGRAVE as u64);
    let mut g1 = duel::select_matching_card(d, turnp, None, turnp, LOCATION_HAND, 0, 2, 2, false, NO_EX).nil();
    duel::confirm_cards(d, 1 - turnp, g1.clone());
    duel::hint(d, HINT_SELECTMSG as u8, 1 - turnp, HINTMSG_TOGRAVE as u64);
    let g2 = duel::select_matching_card(d, 1 - turnp, None, 1 - turnp, LOCATION_HAND, 0, 2, 2, false, NO_EX).nil();
    group::add_card(&mut g1, &g2);
    if !g1.is_empty() && duel::send_to_grave(d, g1.clone(), REASON_EFFECT, None, None) > 0 {
        let og = duel::get_operated_group(d);
        if !group::is_exists(d, &og, &|d, c| card::is_location(d, c, LOCATION_GRAVE), 1, NO_EX) {
            return;
        }
        duel::break_effect(d);
        duel::draw(d, turnp, 2, REASON_EFFECT);
        duel::draw(d, 1 - turnp, 2, REASON_EFFECT);
    }
}
