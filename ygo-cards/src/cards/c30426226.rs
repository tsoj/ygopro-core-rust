//! Switcheroroo
use super::prelude::*;

pub const ID: u32 = 30426226;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_CONTROL);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    !card::is_able_to_change_controler(d, c)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let g = duel::get_field_group(d, tp, LOCATION_MZONE, LOCATION_MZONE);
    let g1 = group::filter(d, &g, &|d, x| card::is_controler(d, x, tp), NO_EX);
    let g2 = group::filter(d, &g, &|d, x| card::is_controler(d, x, 1 - tp), NO_EX);
    if a.chk == Some(0) {
        return !g.is_empty() && g1.len() < 6 && g1.len() == g2.len() && group::filter_count(d, &g, &filter, NO_EX) == 0;
    }
    let cnt = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(g), cnt, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let g1 = duel::get_field_group(d, a.tp, LOCATION_MZONE, 0);
    let g2 = duel::get_field_group(d, a.tp, 0, LOCATION_MZONE);
    duel::swap_control(d, g1, g2, 0, 0);
}
