//! Detonation Code
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 41348446;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TOPOLOGIC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    card::register_effect(d, c, e1, false);
    // Special Summon
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(spcon)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
    // Set itself from banished
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e3, EVENT_REMOVE);
    effect::set_operation(d, e3, Some(operation(setreg)));
    card::register_effect(d, c, e3, false);
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 1));
    effect::set_category(d, e4, CATEGORY_SET);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_range(d, e4, LOCATION_REMOVED);
    effect::set_code(d, e4, EVENT_PHASE | PHASE_STANDBY);
    effect::set_count_limit(d, e4, 1, ID, 1, 0);
    effect::set_condition(d, e4, Some(condition(setcon)));
    effect::set_target(d, e4, Some(cost_target(settg)));
    effect::set_operation(d, e4, Some(operation(setop)));
    effect::set_label_object(d, e4, Value::Effect(e3));
    card::register_effect(d, c, e4, false);
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_type(d, c, TYPE_LINK) && card::is_set_card(d, c, &[SET_TOPOLOGIC as u16]) && card::is_faceup(d, c)
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_attribute(d, c, ATTRIBUTE_DARK)
        && card::is_race(d, c, RACE_DRAGON | RACE_MACHINE | RACE_CYBERSE)
        && !card::is_type(d, c, TYPE_LINK)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && spfilter(d, chkc, e, tp);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) {
            duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
fn setreg(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    if card::is_previous_location(d, h, LOCATION_SZONE) {
        let turn = duel::get_turn_count(d, None) as i64;
        effect::set_label(d, a.e, &[turn]);
        card::register_flag_effect(d, h, ID, RESETS_STANDARD_PHASE_END, 0, 2, 0, 0);
    }
}
fn setcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let e3 = effect::get_label_object(d, a.e).as_effect().nil();
    effect::get_label(d, e3) != duel::get_turn_count(d, None) as i64 && card::get_flag_effect(d, c, ID) > 0
}
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_s_setable(d, c, false, None);
    }
    card::reset_flag_effect(d, c, ID);
    true
}
fn setop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !(card::is_relate_to_effect(d, c, a.e) && card::is_s_setable(d, c, false, None)) {
        return;
    }
    duel::s_set(d, a.tp, c, None, true);
}
