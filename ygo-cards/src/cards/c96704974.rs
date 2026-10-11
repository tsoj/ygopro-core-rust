//! Boogie Trap
use super::prelude::*;

pub const ID: u32 = 96704974;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Set 1 Trap Card from the GY
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SET);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, None, None)), a.tp, LOCATION_HAND, 0, 2, c.into());
    }
    duel::discard_hand(d, a.tp, None, 2, 2, REASON_COST | REASON_DISCARD, c.into());
    true
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsTrap() and c:IsSSetable()
    card::is_trap(d, c) && card::is_s_setable(d, c, false, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && filter(d, chkc);
    }
    let mut ct = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    let h = effect::get_handler(d, a.e).nil();
    if effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) && !card::is_location(d, h, LOCATION_SZONE) {
        ct -= 1;
    }
    if a.chk == Some(0) {
        return ct > 0 && duel::is_existing_target(d, Some(&filter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    let g = duel::select_target(d, tp, Some(&filter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, g, 1, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) && card::is_s_setable(d, tc, false, None) {
        duel::s_set(d, tp, CardSet::from_card(tc), None, true);
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        effect::set_description(d, e1, aux::stringid(ID, 0));
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_TRAP_ACT_IN_SET_TURN);
        effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE, 0);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e1, false);
    }
}
