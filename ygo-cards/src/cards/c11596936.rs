//! Reckoned Power
use super::prelude::*;

pub const ID: u32 = 11596936;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[36623431], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, &[36623431]) && !card::is_public(d, c)
}
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONFIRM as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&cfilter), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    duel::confirm_cards(d, 1 - a.tp, g);
    duel::shuffle_hand(d, a.tp);
    true
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_facedown(d, c) && card::is_spell_trap(d, c)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    let sg = duel::get_matching_group(d, Some(&filter), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    let n = sg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(sg), n, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let sg = duel::get_matching_group(d, Some(&filter), a.tp, 0, LOCATION_ONFIELD, c.into());
    duel::destroy(d, sg, REASON_EFFECT, None, None);
}
