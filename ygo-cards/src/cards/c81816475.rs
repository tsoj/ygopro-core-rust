//! Overlay Eater
use super::prelude::*;

pub const ID: u32 = 81816475;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // (unnamed effect)
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_GRAVE);
    effect::set_cost(d, e1, Some(cost::self_banish()));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::get_overlay_count(d, tp, 0, 1, None) != 0 && duel::is_existing_matching_card(d, Some(&filter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let g1 = duel::get_overlay_group(d, tp, 0, 1, None);
    let g2 = duel::get_matching_group(d, Some(&filter), tp, LOCATION_MZONE, 0, NO_EX);
    if g1.is_empty() || g2.is_empty() {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
    let mg = group::select(d, &g1, tp, 1, 1, false, NO_EX).nil();
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 1));
    let sel = group::select(d, &g2, tp, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel).nil();
    let mc = group::get_first(d, &mg).nil();
    let oc = card::get_overlay_target(d, mc).nil();
    duel::overlay(d, tc, mg, false);
    duel::raise_single_event(d, oc, EVENT_DETACH_MATERIAL, Some(a.e), 0, 0, 0, 0);
}
