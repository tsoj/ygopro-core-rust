//! Card of Sanctity
use super::prelude::*;

pub const ID: u32 = 42664989;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DRAW);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn removable(d: &mut Duel, x: CardId) -> bool {
    card::is_able_to_remove_as_cost(d, x, None)
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&removable), tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, c.into());
    }
    let g = duel::get_matching_group(d, Some(&removable), tp, LOCATION_HAND | LOCATION_ONFIELD, 0, c.into());
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, 0);
    }
    let ht = duel::get_field_group_count(d, tp, LOCATION_HAND, 0);
    duel::set_target_player(d, tp);
    duel::set_target_param(d, (2 - ht) as u32);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 2 - ht);
    true
}
fn op(d: &mut Duel, _a: &EffArgs) {
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    let ht = duel::get_field_group_count(d, p, LOCATION_HAND, 0);
    if ht < 2 {
        duel::draw(d, p, (2 - ht) as u32, REASON_EFFECT);
    }
}
