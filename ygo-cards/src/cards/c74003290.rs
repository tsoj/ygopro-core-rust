//! Lost Wind
use super::prelude::*;

pub const ID: u32 = 74003290;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Targeted monster has its effects negated, also halve its original ATK
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_DISABLE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, Some(TIMING_DAMAGE_STEP | TIMINGS_CHECK_MONSTER));
    effect::set_condition(d, e1, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Set itself from GY
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SET);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_condition(d, e2, Some(condition(setcon)));
    effect::set_target(d, e2, Some(cost_target(settg)));
    effect::set_operation(d, e2, Some(operation(setop)));
    card::register_effect(d, c, e2, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_special_summoned(d, c)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_NEGATE as u64);
    duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        duel::negate_related_chain(d, tc, RESET_TURN_SET);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_DISABLE);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        effect::set_value(d, e2, RESET_TURN_SET as i64);
        card::register_effect(d, tc, e2, false);
        duel::adjust_instantly(d, Some(tc));
        // local e3=e1:Clone()
        let e3 = effect::clone(d, e1);
        effect::set_code(d, e3, EFFECT_SET_BASE_ATTACK);
        // tc:GetBaseAttack()/2 is a float division in Lua
        let base = card::get_base_attack(d, tc);
        effect::set_value(d, e3, lua_int(base as f64 / 2.0));
        card::register_effect(d, tc, e3, false);
    }
}
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_summon_location(d, c, LOCATION_EXTRA) && card::is_previous_controler(d, c, 1 - tp)
}
fn setcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, tp), 1, NO_EX)
}
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_s_setable(d, c, false, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn setop(d: &mut Duel, a: &EffArgs) {
    let (c, e) = (effect::get_handler(d, a.e).nil(), a.e);
    if card::is_relate_to_effect(d, c, e) && card::is_s_setable(d, c, false, None) {
        duel::s_set(d, a.tp, CardSet::from_card(c), None, true);
        // Banish it if it leaves the field
        let e1 = effect::create_effect(d, c);
        effect::set_description(d, e1, 3300);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT, 1);
        effect::set_value(d, e1, LOCATION_REMOVED as i64);
        card::register_effect(d, c, e1, false);
    }
}
