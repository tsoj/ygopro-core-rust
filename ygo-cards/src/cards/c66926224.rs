//! The Law of the Normal
use super::prelude::*;

pub const ID: u32 = 66926224;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_HANDES);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_NORMAL) && card::is_level_below(d, c, 2)
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 5, NO_EX)
}
fn dfilter(d: &mut Duel, c: CardId) -> bool {
    !(card::is_faceup(d, c) && card::is_type(d, c, TYPE_NORMAL) && card::is_level_below(d, c, 2))
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, None, a.tp, LOCATION_HAND, 0, 1, c.into())
            && duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND) > 0
            && duel::is_existing_matching_card(d, Some(&dfilter), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, c.into());
    }
    let g = duel::get_matching_group(d, Some(&dfilter), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, c.into());
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, PLAYER_ALL as u8, 1);
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let g1 = duel::get_field_group(d, a.tp, LOCATION_HAND, LOCATION_HAND);
    duel::send_to_grave(d, g1, REASON_EFFECT | REASON_DISCARD, None, None);
    let g2 = duel::get_matching_group(d, Some(&dfilter), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    duel::destroy(d, g2, REASON_EFFECT, None, None);
}
