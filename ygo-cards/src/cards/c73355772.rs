//! Dogmatika Fleurdelis, the Thunderbolt
use super::prelude::*;

pub const ID: u32 = 73355772;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DOGMATIKA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card from your hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_MAIN_END));
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(spcon)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Set 1 "Dogmatika" Trap from your Deck
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SET);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let handler = effect::get_handler(d, a.e).nil();
        card::is_summon_location(d, handler, LOCATION_HAND)
    })));
    effect::set_target(d, e2, Some(cost_target(settg)));
    effect::set_operation(d, e2, Some(operation(setop)));
    card::register_effect(d, c, e2, false);
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    duel::is_main_phase(d, None)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_summon_location(d, c, LOCATION_EXTRA)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_DOGMATIKA as u16]) && card::is_trap(d, c) && card::is_s_setable(d, c, false, None)
}
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&setfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
fn actcondfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_type(d, c, TYPE_EXTRA) && card::is_attack_above(d, c, 2500) && card::is_faceup(d, c)
}
fn setop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    let sc = duel::select_matching_card(d, tp, Some(&setfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if let Some(sc) = group::get_first(d, &sc) {
        if duel::s_set(d, tp, sc, None, true) > 0 {
            // It can be activated this turn, while your opponent controls a Fusion, Synchro, Xyz, or Link Monster with 2500 or more ATK
            let handler = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, handler);
            effect::set_description(d, e1, aux::stringid(ID, 2));
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE + EFFECT_FLAG_CLIENT_HINT, 0);
            effect::set_code(d, e1, EFFECT_TRAP_ACT_IN_SET_TURN);
            effect::set_condition(d, e1, Some(condition(move |d, _a| {
                duel::is_existing_matching_card(d, Some(&actcondfilter), tp, 0, LOCATION_MZONE, 1, NO_EX)
            })));
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            card::register_effect(d, sc, e1, false);
        }
    }
}
