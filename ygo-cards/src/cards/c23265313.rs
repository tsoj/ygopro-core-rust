//! Cost Down
use super::prelude::*;

pub const ID: u32 = 23265313;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn discardable(d: &mut Duel, c: CardId) -> bool {
    card::is_discardable(d, c, None, None)
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&discardable), a.tp, LOCATION_HAND, 0, 1, c.into());
    }
    duel::discard_hand(d, a.tp, Some(&discardable), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
fn hlvfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_level_above(d, c, 1) && card::is_controler(d, c, tp)
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let fg = duel::get_field_group(d, a.tp, LOCATION_HAND, 0);
    let hg = group::filter(d, &fg, &|d, c| card::is_level_above(d, c, 1), NO_EX);
    let _tc = group::get_first(d, &hg);
    for tc in group::iter(d, &hg) {
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
        effect::set_value(d, e1, -2);
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD_PHASE_END & !RESET_TOFIELD), 1);
        card::register_effect(d, tc, e1, false);
    }
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_TO_HAND);
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    effect::set_operation(d, e2, Some(operation(hlvop)));
    duel::register_effect(d, e2, a.tp);
}
fn hlvop(d: &mut Duel, a: &EffArgs) {
    let eg = a.eg();
    let hg = group::filter(d, &eg, &|d, c| hlvfilter(d, c, a.tp), NO_EX);
    let _tc = group::get_first(d, &hg);
    for tc in group::iter(d, &hg) {
        let c = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
        effect::set_value(d, e1, -2);
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD_PHASE_END & !RESET_TOFIELD), 1);
        card::register_effect(d, tc, e1, false);
    }
}
