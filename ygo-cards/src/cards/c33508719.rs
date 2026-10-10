//! Morphing Jar
use super::prelude::*;

pub const ID: u32 = 33508719;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // flip
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_HANDES | CATEGORY_DRAW);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, PLAYER_ALL as u8, 0);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, PLAYER_ALL as u8, 5);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_field_group(d, a.tp, LOCATION_HAND, LOCATION_HAND);
    if !g.is_empty() {
        duel::send_to_grave(d, g, REASON_DISCARD | REASON_EFFECT, None, None);
    }
    duel::break_effect(d);
    duel::draw(d, a.tp, 5, REASON_EFFECT);
    duel::draw(d, 1 - a.tp, 5, REASON_EFFECT);
}
