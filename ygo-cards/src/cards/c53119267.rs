//! Magical Thorn
use super::prelude::*;

pub const ID: u32 = 53119267;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // damage
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_operation(d, e2, Some(operation(damop)));
    card::register_effect(d, c, e2, false);
}
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_previous_location(d, c, LOCATION_HAND) && card::get_controler(d, c) != tp && !card::is_previous_controler(d, c, tp) && card::is_reason(d, c, REASON_DISCARD)
}
fn damop(d: &mut Duel, a: &EffArgs) {
    let eg = a.eg.clone().nil();
    let ct = group::filter_count(d, &eg, &|d, c| filter(d, c, a.tp), NO_EX);
    duel::damage(d, 1 - a.tp, (ct * 500) as i64, REASON_EFFECT, false, None);
}
