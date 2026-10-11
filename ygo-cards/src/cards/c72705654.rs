//! Tachyon Cloudragon
use super::prelude::*;

pub const ID: u32 = 72705654;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_TACHYON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon itself when added to the hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_TO_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(selfs_cond)));
    effect::set_target(d, e1, Some(cost_target(selfsptg)));
    effect::set_operation(d, e1, Some(operation(selfspop)));
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "Tachyon" monster from your Deck or GY
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_cost(d, e2, Some(cost::self_tribute()));
    effect::set_target(d, e2, Some(cost_target(dsptg)));
    effect::set_operation(d, e2, Some(operation(dspop)));
    card::register_effect(d, c, e2, false);
    // Attach this card to a Dragon Xyz Monster as material
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 2));
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e3, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    effect::set_range(d, e3, LOCATION_HAND | LOCATION_GRAVE);
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    effect::set_target(d, e3, Some(cost_target(attachtg)));
    effect::set_operation(d, e3, Some(operation(attachop)));
    card::register_effect(d, c, e3, false);
}
fn selfs_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    !card::is_reason(d, h, REASON_DRAW)
}
fn selfsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
fn selfspop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, CardSet::from_card(c), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
fn dspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_TACHYON as u16]) && !card::is_code(d, c, &[ID]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn dsptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        let _ = duel::get_m_zone_count(d, a.tp, c.into(), None, None, None);
        return duel::is_existing_matching_card(d, Some(&|d, x| dspfilter(d, x, a.e, a.tp)), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
fn dspop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&|d, c| aux::necro_valley_filter(|d, x| dspfilter(d, x, e, tp))(d, c)),
        tp,
        LOCATION_DECK | LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    if !g.is_empty() {
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
fn attachfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_summon_player(d, c, tp)
        && card::is_race(d, c, RACE_DRAGON)
        && card::is_type(d, c, TYPE_XYZ)
        && card::is_faceup(d, c)
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::is_location(d, c, LOCATION_MZONE)
}
fn attachtg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        let eg = a.eg.clone().nil();
        return group::is_contains(&eg, chkc) && attachfilter(d, chkc, a.e, a.tp);
    }
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return group::is_exists(d, &eg, &|d, c| attachfilter(d, c, a.e, a.tp), 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    let eg = a.eg.clone().nil();
    let g = group::filter_select(d, &eg, a.tp, &|d, c| attachfilter(d, c, a.e, a.tp), 1, 1, false, NO_EX).nil();
    duel::set_target_card(d, g);
    let c = effect::get_handler(d, a.e).nil();
    if card::is_location(d, c, LOCATION_GRAVE) {
        duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(c)), 1, a.tp, 0);
    }
    true
}
fn attachop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d);
    if card::is_relate_to_effect(d, c, a.e) && card::is_relate_to_effect(d, tc.nil(), a.e) && !card::is_immune_to_effect(d, tc.nil(), a.e) {
        duel::overlay(d, tc.nil(), c, false);
    }
}
