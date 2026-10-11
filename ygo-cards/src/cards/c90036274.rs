//! Clear Wing Fast Dragon
use super::prelude::*;

pub const ID: u32 = 90036274;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SPEEDROID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Synchro summon
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner_ex(|d, x, sc, st, tp| card::is_attribute_ctx(d, x, ATTRIBUTE_WIND, sc, st, Some(tp)))), 1, 99, None, None, None, None, None);
    card::enable_revive_limit(d, c);
    // Pendulum attributes
    pendulum::add_procedure(d, c, Some(false), None);
    // Special summon from the pendulum zone
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_PZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Negate effect and decrease ATK
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_ATKCHANGE | CATEGORY_DISABLE);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET | EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_hint_timing(d, e2, TIMING_DAMAGE_STEP, Some(TIMING_DAMAGE_STEP | TIMINGS_CHECK_MONSTER));
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    effect::set_target(d, e2, Some(cost_target(distg)));
    effect::set_operation(d, e2, Some(operation(disop)));
    card::register_effect(d, c, e2, false);
    // Place itself in the pendulum zone
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 2));
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e3, EVENT_DESTROYED);
    effect::set_condition(d, e3, Some(condition(pencon)));
    effect::set_target(d, e3, Some(cost_target(pentg)));
    effect::set_operation(d, e3, Some(operation(penop)));
    card::register_effect(d, c, e3, false);
}
fn cfilter1(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_SPEEDROID as u16]) && card::is_type(d, c, TYPE_TUNER)
}
fn cfilter2(d: &mut Duel, c: CardId) -> bool {
    !card::is_type(d, c, TYPE_TUNER) || !card::is_has_effect(d, c, EFFECT_NONTUNER, None).is_empty()
}
fn filter_fn(d: &mut Duel, c: CardId, f: fn(&mut Duel, CardId) -> bool) -> bool {
    card::is_faceup(d, c) && card::is_level_above(d, c, 0) && card::is_able_to_grave_as_cost(d, c) && f(d, c)
}
fn chk(d: &mut Duel, c: CardId, sg: &CardSet) -> bool {
    cfilter1(d, c) && group::is_exists(d, sg, &|d, x| cfilter2(d, x), 1, c.into())
}
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    let mmz = aux::chkf_mmz(1)(d, sg, e, tp, mg, None).0;
    let res = mmz && group::is_exists(d, sg, &|d, x| chk(d, x, sg), 1, NO_EX) && group::check_with_sum_equal(d, sg, &|d, x| card::get_level(d, x) as i64, 7, 2, 2).0;
    (res, false)
}
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let g1 = duel::get_matching_group(d, Some(&|d, x| filter_fn(d, x, cfilter1)), tp, LOCATION_MZONE, 0, NO_EX);
    let g2 = duel::get_matching_group(d, Some(&|d, x| filter_fn(d, x, cfilter2)), tp, LOCATION_MZONE, 0, NO_EX);
    let mut gg = g1.clone();
    group::add_card(&mut gg, &g2);
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > -2
            && !g1.is_empty()
            && !g2.is_empty()
            && matches!(aux::select_unselect_group(d, &gg, Some(e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false), Err(true));
    }
    let sg = aux::select_unselect_group(d, &gg, Some(e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_TOGRAVE as u64), None, None, false).unwrap_or_default();
    duel::send_to_grave(d, sg, REASON_COST, None, None);
    true
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, _tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if a.chk == Some(0) {
        return card::is_can_be_special_summoned(d, c, e, 0, a.tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if card::is_relate_to_effect(d, c, e) {
        duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
fn disfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_summon_location(d, c, LOCATION_EXTRA) && !(card::get_attack(d, c) == 0 && card::is_disabled(d, c))
}
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && disfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&disfilter), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    let _ = duel::select_target(d, tp, Some(&disfilter), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
fn disop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d).nil();
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        effect::set_value(d, e1, 0);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
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
fn pencon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    (a.r & (REASON_EFFECT + REASON_BATTLE)) != 0 && card::is_previous_location(d, c, LOCATION_MZONE) && card::is_faceup(d, c)
}
fn pentg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::check_pendulum_zones(d, a.tp);
    }
    true
}
fn penop(d: &mut Duel, a: &EffArgs) {
    if !duel::check_pendulum_zones(d, a.tp) {
        return;
    }
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::move_to_field(d, c, a.tp, a.tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
    }
}
