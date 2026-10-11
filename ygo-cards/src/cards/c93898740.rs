//! Unchained Syncretism
use super::prelude::*;

pub const ID: u32 = 93898740;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_UNCHAINED_SOUL, SET_UNCHAINED], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Target 2 face-up monsters on the field, including an "Unchained Soul" Link Monster you control; immediately after this effect resolves, Link Summon 1 Fiend monster using only those 2 monsters
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(linksumtg)));
    effect::set_operation(d, e1, Some(operation(linksumop)));
    effect::set_hint_timing(d, e1, 0, Some((TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E) as u32));
    card::register_effect(d, c, e1, false);
    // If this Set card is destroyed by card effect: You can Special Summon 1 "Unchained" monster from your Deck
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_DESTROYED);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(deckspcon)));
    effect::set_target(d, e2, Some(cost_target(decksptg)));
    effect::set_operation(d, e2, Some(operation(deckspop)));
    card::register_effect(d, c, e2, false);
}
fn unchainedsoulfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_UNCHAINED_SOUL as u16]) && card::is_link_monster(d, c) && card::is_controler(d, c, tp)
}
fn linkfilter(d: &mut Duel, c: CardId, mg: &CardSet) -> bool {
    card::is_race(d, c, RACE_FIEND) && card::is_link_summonable(d, c, None, Some(mg.clone()), 2, 2)
}
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    let a = group::is_exists(d, sg, &|d, c| unchainedsoulfilter(d, c, tp), 1, NO_EX);
    let b = a && duel::is_existing_matching_card(d, Some(&|d, c| linkfilter(d, c, sg)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    (b, false)
}
fn linksumtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chkc().is_some() {
        return false;
    }
    let g = duel::get_target_group(
        d,
        Some(&|d, c| card::is_faceup(d, c) && card::is_can_be_link_material(d, c, None, None)),
        tp,
        LOCATION_MZONE,
        LOCATION_MZONE,
        NO_EX,
    );
    if a.chk == Some(0) {
        return g.len() >= 2 && matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false), Err(true));
    }
    let tg = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_TARGET as u64), None, None, false).unwrap_or_default();
    duel::set_target_card(d, tg);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
fn linkmatimmunefilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    card::is_faceup(d, c) && card::is_can_be_link_material(d, c, None, None) && !card::is_immune_to_effect(d, c, e)
}
fn linksumop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let mut tg = duel::get_target_cards(d, e).nil();
    group::match_(d, &mut tg, &|d, c| linkmatimmunefilter(d, c, e), NO_EX);
    if tg.len() == 2 {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let sel = duel::select_matching_card(d, tp, Some(&|d, c| linkfilter(d, c, &tg)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        if let Some(link) = group::get_first(d, &sel) {
            duel::link_summon(d, tp, link, Some(tg.clone()), Some(tg.clone()), 2, 2);
        }
    }
}
fn deckspcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, c, LOCATION_ONFIELD) && card::is_previous_position(d, c, POS_FACEDOWN) && card::is_reason(d, c, REASON_EFFECT)
}
fn deckspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_UNCHAINED as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn decksptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| deckspfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
fn deckspop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| deckspfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
