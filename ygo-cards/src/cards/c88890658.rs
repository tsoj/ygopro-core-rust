//! Baelgrill de Nouvelles
use super::prelude::*;

pub const ID: u32 = 88890658;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[30243636], listed_series: &[SET_RECIPE, SET_NOUVELLES], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Register that it was Special Summoned by the effect of a "Nouvelles" monster
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e0, EVENT_SPSUMMON_SUCCESS);
    effect::set_condition(d, e0, Some(condition(regcon)));
    effect::set_operation(d, e0, Some(operation(regop)));
    card::register_effect(d, c, e0, false);
    // Negate the effects of all cards the opponent controls
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DISABLE + CATEGORY_RELEASE);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(negtg)));
    effect::set_operation(d, e1, Some(operation(negop)));
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "Hungry Burger" from your hand or Deck
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_RELEASE + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER_E));
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
}
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    match a.re {
        Some(re) => effect::is_monster_effect(d, re) && effect::is_card_setcode(d, re, &[SET_NOUVELLES as u16]),
        None => false,
    }
}
fn regop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESET_EVENT | (RESETS_STANDARD & !RESET_TEMP_REMOVE), EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 2));
}
fn releasable(d: &mut Duel, c: CardId) -> bool {
    card::is_releasable_by_effect(d, c, None, None)
}
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&card::is_negatable), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    let g = duel::get_matching_group(d, Some(&card::is_negatable), tp, 0, LOCATION_ONFIELD, NO_EX);
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(g), n, 0, 0);
    let h = effect::get_handler(d, a.e).nil();
    if card::is_nouvelles_summoned(d, h) {
        let rg = duel::get_matching_group(d, Some(&releasable), tp, 0, LOCATION_MZONE, NO_EX);
        let n = rg.len() as u8;
        duel::set_operation_info(d, 0, CATEGORY_RELEASE, Some(rg), n, 0, 0);
    }
    true
}
fn negop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let g = duel::get_matching_group(d, Some(&card::is_negatable), tp, 0, LOCATION_ONFIELD, NO_EX);
    let g = group::filter(d, &g, &|d, c| card::is_can_be_disabled_by_effect(d, c, e, true), NO_EX);
    if g.len() == 0 {
        return;
    }
    // Negate the effects of all opponent's cards
    let c = effect::get_handler(d, e).nil();
    for tc in group::iter(d, &g) {
        duel::negate_related_chain(d, tc, RESET_TURN_SET);
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_DISABLE);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e1, false);
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        effect::set_value(d, e2, RESET_TURN_SET as i64);
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e2, false);
    }
    // Tribute as many monsters as possible
    if card::is_nouvelles_summoned(d, c) {
        let rg = duel::get_matching_group(d, Some(&releasable), tp, 0, LOCATION_MZONE, NO_EX);
        if rg.len() == 0 {
            return;
        }
        duel::break_effect(d);
        duel::release(d, rg, REASON_EFFECT, None);
    }
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_attack_pos(d, c) && releasable(d, c)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_code(d, c, &[30243636]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, true, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && cfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&cfilter), tp, 0, LOCATION_MZONE, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RELEASE as u64);
    let g = duel::select_target(d, tp, Some(&cfilter), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_RELEASE, g, 1, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, e)
        && duel::release(d, tc, REASON_EFFECT, None) > 0
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
    {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        if g.len() > 0 {
            duel::special_summon(d, g, 0, tp, tp, false, true, POS_FACEUP as u8, None);
        }
    }
}
