//! Magical Musket - Cross-Domination
use super::prelude::*;

pub const ID: u32 = 93356623;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MAGICAL_MUSKET], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE + CATEGORY_DISABLE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, Some(TIMING_DAMAGE_STEP | TIMINGS_CHECK_MONSTER));
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_MAGICAL_MUSKET as u16]))), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
        && aux::stat_change_damage_step_condition(d)
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && !(card::get_attack(d, c) == 0 && card::get_defense(d, c) == 0 && card::is_disabled(d, c))
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    let _ = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d);
    if let Some(tc) = tc {
        if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
            let e0 = effect::create_effect(d, c);
            effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e0, EFFECT_SET_ATTACK_FINAL);
            effect::set_value(d, e0, 0);
            effect::set_reset(d, e0, RESETS_STANDARD_PHASE_END, 1);
            card::register_effect(d, tc, e0, false);
            let e1 = effect::clone(d, e0);
            effect::set_code(d, e1, EFFECT_SET_DEFENSE_FINAL);
            card::register_effect(d, tc, e1, false);
            duel::negate_related_chain(d, tc, RESET_TURN_SET);
            let e2 = effect::create_effect(d, c);
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e2, EFFECT_DISABLE);
            effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
            card::register_effect(d, tc, e2, false);
            let e3 = effect::create_effect(d, c);
            effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e3, EFFECT_DISABLE_EFFECT);
            effect::set_value(d, e3, RESET_TURN_SET as i64);
            effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
            card::register_effect(d, tc, e3, false);
        }
    }
}
