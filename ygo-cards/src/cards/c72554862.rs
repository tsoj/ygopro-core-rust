//! War Rock Skyler
use super::prelude::*;

pub const ID: u32 = 72554862;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_WAR_ROCK],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Gains 100 ATK per opponent's monsters
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_value_fn(d, e1, effect_card_value(atkval));
    card::register_effect(d, c, e1, false);
    // If your Earth Warrior monster battles
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_ATKCHANGE + CATEGORY_SPECIAL_SUMMON);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_MZONE);
    let timing = TIMING_BATTLE_PHASE | TIMING_BATTLE_END | TIMING_ATTACK | TIMING_BATTLE_START | TIMING_DAMAGE_STEP;
    effect::set_hint_timing(d, e2, timing as u32, Some(timing as u32));
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(spcon)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
    // Condition check for the Quick Effect
    aux::global_check(d, ID, move |d| {
        let ge1 = effect::create_effect(d, c);
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, ge1, EVENT_BATTLED);
        effect::set_operation(d, ge1, Some(operation(checkop)));
        duel::register_effect(d, ge1, 0);
    });
}
fn checkfilter(d: &mut Duel, c: Option<CardId>) -> bool {
    match c {
        Some(c) => card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_EARTH) && card::is_race(d, c, RACE_WARRIOR as u64),
        None => false,
    }
}
fn checkop(d: &mut Duel, _a: &EffArgs) {
    if !duel::is_damage_calculated(d) {
        return;
    }
    let (bc0, bc1) = duel::get_battle_monster(d, 0);
    if checkfilter(d, bc0) {
        let p = card::get_controler(d, bc0.nil());
        duel::register_flag_effect(d, p, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    }
    if checkfilter(d, bc1) {
        let p = card::get_controler(d, bc1.nil());
        duel::register_flag_effect(d, p, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    }
}
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    let p = card::get_controler(d, c);
    duel::get_field_group_count(d, p, 0, LOCATION_MZONE) as i64 * 100
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    duel::is_battle_phase(d, None) && duel::get_flag_effect(d, tp, ID) > 0 && (duel::get_current_phase(d) != PHASE_DAMAGE || !duel::is_damage_calculated(d))
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_level_below(d, c, 5) && card::is_race(d, c, RACE_WARRIOR as u64) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && spfilter(d, chkc, e, tp);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_WAR_ROCK as u16]))), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    let sc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, sc, a.e) {
        duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
    let atkg = duel::get_matching_group(d, Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_WAR_ROCK as u16]))), tp, LOCATION_MZONE, 0, NO_EX);
    for tc in group::iter(d, &atkg) {
        // Increase ATK
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END | RESET_OPPO_TURN, 1);
        effect::set_value(d, e1, 200);
        card::register_effect(d, tc, e1, false);
    }
    // Cannot attack directly with Level 5 or lower monsters
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_code(d, e2, EFFECT_CANNOT_DIRECT_ATTACK);
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_level_below(d, c, 5))));
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e2, tp);
    aux::register_client_hint(d, c, 0, tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
}
