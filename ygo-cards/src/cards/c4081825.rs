//! Chaos Infinity
use super::prelude::*;

pub const ID: u32 = 4081825;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MEKLORD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_POSITION + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_MEKLORD as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_defense_pos(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_defense_pos(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    if g.is_empty() {
        return;
    }
    duel::change_position(d, g, POS_FACEUP_ATTACK as u8, None, None, None, false);
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let sg = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, tp))), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    if let Some(tc) = group::get_first(d, &sg) {
        duel::break_effect(d);
        duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_DISABLE);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e1, true);
        let e2 = effect::create_effect(d, h);
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e2, true);
        duel::special_summon_complete(d);
        let e3 = effect::create_effect(d, h);
        effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_range(d, e3, LOCATION_MZONE);
        effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
        effect::set_operation(d, e3, Some(operation(desop)));
        effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
        effect::set_count_limit(d, e3, 1, 0, 0, 0);
        card::register_effect(d, tc, e3, false);
    }
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    duel::destroy(d, h, REASON_EFFECT, None, None);
}
