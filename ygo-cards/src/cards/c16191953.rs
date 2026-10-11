//! Master Gig
use super::prelude::*;

pub const ID: u32 = 16191953;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // destroy
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(1000))));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn psychic_faceup(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_race(d, c, RACE_PSYCHIC)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::get_matching_group_count(d, Some(&psychic_faceup), tp, LOCATION_MZONE, 0, NO_EX) <= duel::get_matching_group_count(d, None, tp, 0, LOCATION_MZONE, NO_EX);
    }
    let ct = duel::get_matching_group_count(d, Some(&psychic_faceup), tp, LOCATION_MZONE, 0, NO_EX);
    let dg = duel::get_matching_group(d, None, tp, 0, LOCATION_MZONE, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg), ct as u8, 0, 0);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let ct = duel::get_matching_group_count(d, Some(&psychic_faceup), tp, LOCATION_MZONE, 0, NO_EX);
    let g = duel::get_matching_group(d, None, tp, 0, LOCATION_MZONE, NO_EX);
    if ct > g.len() {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    let dg = group::select(d, &g, tp, ct as u16, ct as u16, false, NO_EX).nil();
    duel::hint_selection(d, dg.clone(), true);
    duel::destroy(d, dg, REASON_EFFECT, None, None);
}
