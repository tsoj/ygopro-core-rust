//! Charmer Quartet in Bloom
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 27519978;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 2+ monsters
    link::add_procedure(d, c, None, 2, None, None, None, None);
    // atk up
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_value_fn(d, e1, effect_value(atk_value));
    card::register_effect(d, c, e1, false);
    // special summon
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e2, EFFECT_FLAG_NO_TURN_RESET + EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(e2cond)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    card::register_effect(d, c, e2, false);
}
/// `function(e,c) return 300*Duel.GetMatchingGroup(...):GetBinClassCount(Card.GetAttribute) end`
fn atk_value(d: &mut Duel, _e: EffectId) -> i64 {
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x)), 0, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    300 * group::get_bin_class_count(d, &g, &|d, x| card::get_attribute(d, x) as i64) as i64
}
fn e2cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_main_phase(d, None)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_can_be_effect_target(d, c, Some(e)) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.spchk(sg, e, tp)`
fn spchk(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    let t = group::get_class_count(d, sg, &|d, x| card::get_race(d, x) as i64);
    let a = group::get_class_count(d, sg, &|d, x| card::get_attribute(d, x) as i64);
    (((t == 1 && a == sg.len()) || (a == 1 && t == sg.len())), false)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chkc().is_some() {
        return false;
    }
    let g = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, a.e, tp)), tp, LOCATION_GRAVE, 0, NO_EX);
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 >= 2
            && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && matches!(aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(&spchk), 0, tp, None, None, None, false), Err(true));
    }
    let sg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(&spchk), 1, tp, Some(HINTMSG_SPSUMMON as u64), None, None, false)
        .unwrap_or_default();
    duel::set_target_card(d, sg.clone());
    let n = sg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(sg), n, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 2 {
        return;
    }
    let c = effect::get_handler(d, a.e).nil();
    let tg = duel::get_target_cards(d, a.e).nil();
    if tg.len() == 2 && duel::special_summon(d, &tg, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
        let mut g = duel::get_operated_group(d);
        if card::is_relate_to_effect(d, c, a.e) {
            group::add_card(&mut g, &CardSet::from_card(c));
        }
        for tc in group::iter(d, &g) {
            let e1 = effect::create_effect(d, c);
            effect::set_description(d, e1, aux::stringid(ID, 1));
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
            effect::set_code(d, e1, EFFECT_CANNOT_BE_MATERIAL);
            effect::set_value_fn(d, e1, aux::cannotmatfilter(&[SUMMON_TYPE_FUSION, SUMMON_TYPE_SYNCHRO, SUMMON_TYPE_XYZ, SUMMON_TYPE_LINK]));
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 2);
            card::register_effect(d, tc, e1, false);
        }
    }
}
