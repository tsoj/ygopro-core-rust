//! Overlay Owl
use super::prelude::*;

pub const ID: u32 = 59644958;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // detach
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(600))));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::check_remove_overlay_card(d, a.tp, 1, 1, 1, REASON_EFFECT, None);
    }
    true
}
fn overlay_filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::check_remove_overlay_card(d, c, tp, 1, REASON_EFFECT)
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DEATTACHFROM as u64);
    let sg = duel::select_matching_card(d, tp, Some(&|d, c| overlay_filter(d, c, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    if sg.is_empty() {
        return;
    }
    duel::hint_selection(d, sg.clone(), true);
    let tc = group::get_first(d, &sg).nil();
    card::remove_overlay_card(d, tc, tp, 1, 1, REASON_EFFECT);
}
