//! Chronomaly Cabrera Trebuchet
use super::prelude::*;

pub const ID: u32 = 20154092;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CHRONOMALY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atk
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_cost(d, e2, Some(cost_target(cost_fn)));
    effect::set_target(d, e2, Some(cost_target(target)));
    effect::set_operation(d, e2, Some(operation(op)));
    card::register_effect(d, c, e2, false);
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_CHRONOMALY as u16])
}
fn filter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    card::is_faceup(d, c) && card::is_can_be_effect_target(d, c, Some(e))
}
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let dg = duel::get_matching_group(d, Some(&|d, x| filter(d, x, a.e)), a.tp, 0, LOCATION_MZONE, NO_EX);
    let c = effect::get_handler(d, a.e).nil();
    let check = |d: &mut Duel, sg: &CardSet, tp: u8, exg: &CardSet| aux::release_check_target(d, sg, tp, exg, &dg);
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&|d, x| cfilter(d, x)), 1, 1, false, Some(&check), c.into());
    }
    let g = duel::select_release_group_cost(d, a.tp, Some(&|d, x| cfilter(d, x)), 1, 1, false, Some(&check), c.into());
    duel::release(d, g, REASON_COST, None);
    true
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| card::is_faceup(d, x)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    duel::select_target(d, a.tp, Some(&|d, x| card::is_faceup(d, x)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        let c = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        effect::set_value(d, e1, 0);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e1, false);
    }
}
