//! Nemeses Adrastea
use super::prelude::*;

pub const ID: u32 = 45666710;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NEMESES], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon 1 of your "Nemeses" monsters, that is banished or in GY
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_NEMESES as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE | LOCATION_REMOVED) && card::is_controler(d, chkc, a.tp) && spfilter(d, chkc, a.e, a.tp);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, (LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d);
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.SpecialSummonStep(...)~=0: a boolean is never 0, so the branch always runs
            duel::special_summon_step(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
            if card::get_original_level(d, tc) >= 11 {
                // Cannot activate its effects
                let e1 = effect::create_effect(d, c);
                effect::set_description(d, e1, 3302);
                effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                effect::set_code(d, e1, EFFECT_CANNOT_TRIGGER);
                effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
                card::register_effect(d, tc, e1, false);
            }
            duel::special_summon_complete(d);
        }
    }
}
