//! Allure of Darkness
use super::prelude::*;

pub const ID: u32 = 1475311;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DRAW + CATEGORY_REMOVE + CATEGORY_HANDES);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_player_affected_by_effect(d, a.tp, 30459350).is_none() && duel::is_player_can_draw(d, a.tp, 2);
    }
    duel::set_target_player(d, a.tp);
    duel::set_target_param(d, 2);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 2);
    duel::set_possible_operation_info(d, 0, CATEGORY_HANDES, None, 2, a.tp, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    let cnt = info[1].int() as u32;
    duel::draw(d, p, cnt, REASON_EFFECT);
    duel::shuffle_hand(d, p);
    duel::break_effect(d);
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    let sel = duel::select_matching_card(d, p, Some(&|d, c| card::is_attribute(d, c, ATTRIBUTE_DARK)), p, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let g = group::get_first(d, &sel);
    if let Some(g) = g {
        if duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None) == 0 {
            duel::confirm_cards(d, 1 - p, g);
            duel::shuffle_hand(d, p);
        }
    } else {
        let sg = duel::get_field_group(d, p, LOCATION_HAND, 0);
        duel::send_to_grave(d, sg, REASON_EFFECT, None, None);
    }
}
