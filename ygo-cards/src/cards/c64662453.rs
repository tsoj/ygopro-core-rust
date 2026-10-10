//! Iron Call
use super::prelude::*;

pub const ID: u32 = 64662453;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let f = aux::faceup_filter(|d, c| card::is_race(d, c, RACE_MACHINE));
    duel::is_existing_matching_card(d, Some(&f), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_level_below(d, c, 4) && card::is_race(d, c, RACE_MACHINE) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc, a.e, a.tp);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, a.tp, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) && duel::special_summon_step(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) {
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_DISABLE);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e1, true);
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e2, true);
        card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
        duel::special_summon_complete(d);
        let e3 = effect::create_effect(d, c);
        effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
        effect::set_count_limit(d, e3, 1, 0, 0, 0);
        effect::set_property(d, e3, EFFECT_FLAG_IGNORE_IMMUNE, 0);
        effect::set_label_object(d, e3, Value::Card(tc));
        effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 1);
        effect::set_condition(d, e3, Some(condition(descon)));
        effect::set_operation(d, e3, Some(operation(desop)));
        duel::register_effect(d, e3, a.tp);
    }
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    if card::get_flag_effect(d, tc, ID) != 0 {
        true
    } else {
        effect::reset(d, a.e);
        false
    }
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    duel::destroy(d, tc, REASON_EFFECT, None, None);
}
