//! Creature Swap
use super::prelude::*;

pub const ID: u32 = 31036355;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Each player changes control of 1 of their monsters
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_CONTROL);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_able_to_change_controler(d, c)
        && (card::get_sequence(d, c) < 5 || duel::get_location_count(d, card::get_controler(d, c), LOCATION_MZONE as u8, None, None, None).0 > 0)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&filter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, None, 0, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e);
    if !duel::is_existing_matching_card(d, Some(&filter), tp, LOCATION_MZONE, 0, 1, NO_EX)
        || !duel::is_existing_matching_card(d, Some(&filter), tp, 0, LOCATION_MZONE, 1, NO_EX)
    {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONTROL as u64);
    let g1 = duel::select_matching_card(d, tp, Some(&filter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    duel::hint_selection(d, g1.clone(), true);
    duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_CONTROL as u64);
    let g2 = duel::select_matching_card(d, 1 - tp, Some(&filter), 1 - tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    duel::hint_selection(d, g2.clone(), true);
    let c1 = group::get_first(d, &g1).nil();
    let c2 = group::get_first(d, &g2).nil();
    if duel::swap_control(d, c1, c2, 0, 0) {
        // Cannot change their battle positions
        let e1 = effect::create_effect(d, c.nil());
        effect::set_description(d, e1, 3313);
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_CANNOT_CHANGE_POSITION);
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        card::register_effect(d, c1, e1, false);
        let e2 = effect::clone(d, e1);
        card::register_effect(d, c2, e2, false);
    }
}
