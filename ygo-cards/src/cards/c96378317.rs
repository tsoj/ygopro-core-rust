//! Tri-Brigade Rendezvous
use super::prelude::*;

pub const ID: u32 = 96378317;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // ATK up
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    effect::set_target(d, e1, Some(cost_target(atktg)));
    effect::set_operation(d, e1, Some(operation(atkop)));
    card::register_effect(d, c, e1, false);
    // Substitute destruction for a linked beast/beast-warrior/winged-beast monster(s)
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_target(d, e2, Some(cost_target(reptg)));
    effect::set_value_fn(d, e2, effect_card_fn(repval));
    effect::set_operation(d, e2, Some(operation(repop)));
    card::register_effect(d, c, e2, false);
}
fn filter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    card::is_faceup(d, c)
        && card::is_race(d, c, RACES_BEAST_BWARRIOR_WINGB as u64)
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::is_linked(d, c)
}
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc, a.e);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, a.e)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    let n = duel::get_matching_group_count(d, Some(&|d, c| filter(d, c, a.e)), a.tp, LOCATION_MZONE, 0, NO_EX);
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SELECT as u64);
    let _g = duel::select_target(d, a.tp, Some(&|d, c| filter(d, c, a.e)), a.tp, LOCATION_MZONE, 0, 1, n as u16, false, NO_EX);
    true
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let targets = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    let g = group::filter(d, &targets, &|d, c| card::is_relate_to_effect(d, c, a.e), NO_EX);
    if !g.is_empty() {
        let c = effect::get_handler(d, a.e).nil();
        for tc in group::iter(d, &g) {
            op(d, tc, c);
        }
    }
}
fn op(d: &mut Duel, tc: CardId, c: CardId) {
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    effect::set_value(d, e1, 700);
    card::register_effect(d, tc, e1, false);
}
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_faceup(d, c)
        && card::is_race(d, c, RACES_BEAST_BWARRIOR_WINGB as u64)
        && card::is_linked(d, c)
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_controler(d, c, tp)
        && !card::is_reason(d, c, REASON_REPLACE)
        && card::is_reason(d, c, REASON_EFFECT | REASON_BATTLE)
}
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return card::is_able_to_remove(d, c, None, None, None) && group::is_exists(d, &eg, &|d, mc| repfilter(d, mc, a.tp), 1, NO_EX);
    }
    duel::select_effect_yes_no(d, a.tp, c, Some(96))
}
fn repval(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let tp = effect::get_handler_player(d, e);
    repfilter(d, c, tp)
}
fn repop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    duel::remove(d, c, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
