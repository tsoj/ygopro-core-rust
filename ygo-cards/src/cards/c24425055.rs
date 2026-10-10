//! Booby Trap E
use super::prelude::*;

pub const ID: u32 = 24425055;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Set 1 Continuous Trap from the hand or GY
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SET);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn filter1(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_discardable(d, c, None, None)
        && ((setfilter(d, c) && card::is_able_to_grave_as_cost(d, c))
            || duel::is_existing_matching_card(d, Some(&setfilter), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, c.into()))
}
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_continuous_trap(d, c) && card::is_s_setable(d, c, false, None)
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| filter1(d, c, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::discard_hand(d, tp, Some(&|d, c| filter1(d, c, tp)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(setfilter)), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    if let Some(tc) = group::get_first(d, &g) {
        duel::s_set(d, tp, tc, None, true);
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_TRAP_ACT_IN_SET_TURN);
        effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE, 0);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e1, false);
    }
}
