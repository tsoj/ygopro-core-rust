//! Skull Invitation
use super::prelude::*;

pub const ID: u32 = 98139712;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // damage
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_operation(d, e2, Some(operation(op)));
    card::register_effect(d, c, e2, false);
}
fn filter1(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::get_owner(d, c) == 1 - tp
}
fn filter2(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::get_owner(d, c) == tp
}
fn op(d: &mut Duel, a: &EffArgs) {
    let eg = a.eg.clone().nil();
    let d1 = group::filter_count(d, &eg, &|d, c| filter1(d, c, a.tp), NO_EX) as i64 * 300;
    let d2 = group::filter_count(d, &eg, &|d, c| filter2(d, c, a.tp), NO_EX) as i64 * 300;
    duel::damage(d, 1 - a.tp, d1, REASON_EFFECT, true, None);
    duel::damage(d, a.tp, d2, REASON_EFFECT, true, None);
    duel::rd_complete(d);
}
