//! Sextet Summon
use super::prelude::*;

pub const ID: u32 = 99162753;

const TYPES: u32 = TYPE_FUSION | TYPE_RITUAL | TYPE_SYNCHRO | TYPE_XYZ | TYPE_PENDULUM | TYPE_LINK;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_REMOVE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_type(d, c, TYPES) && (card::is_faceup(d, c) || !card::is_location(d, c, LOCATION_MZONE)) && card::is_able_to_remove(d, c, None, None, None)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, rc: u64) -> bool {
    if card::is_location(d, c, LOCATION_EXTRA)
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 == 0
    {
        return false;
    }
    card::get_original_race(d, c) == rc && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    if group::get_class_count(d, sg, &|d, c| card::get_original_race(d, c) as i64) != 1
        || !group::check_different_property_binary(d, sg, &|d, c| card::get_type(d, c) as u64 & TYPES as u64)
    {
        return (false, true);
    }
    let first = group::get_first(d, sg).nil();
    let rc = card::get_original_race(d, first);
    let e = e.nil();
    let res = duel::is_existing_matching_card(
        d,
        Some(&|d, c| spfilter(d, c, e, tp, rc)),
        tp,
        LOCATION_HAND | LOCATION_MZONE | LOCATION_GRAVE,
        0,
        1,
        NO_EX,
    );
    (res, false)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let g = duel::get_matching_group(d, Some(&cfilter), a.tp, LOCATION_HAND | LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
        return matches!(
            aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(6), Some(6), Some(&rescon), 0, a.tp, None, None, None, false),
            Err(true)
        );
    }
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 6, a.tp, (LOCATION_HAND | LOCATION_MZONE | LOCATION_GRAVE) as i32);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_DECK | LOCATION_EXTRA) as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let g = duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_HAND | LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    let rg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(6), Some(6), Some(&rescon), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, false)
        .unwrap_or_default();
    if rg.len() == 6 && duel::remove(d, &rg, POS_FACEUP as u8, REASON_EFFECT, None, None) == 6 {
        duel::break_effect(d);
        let first = group::get_first(d, &rg).nil();
        let rc = card::get_original_race(d, first);
        let mut loc = LOCATION_EXTRA;
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
            loc |= LOCATION_DECK;
        }
        let e = a.e;
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let sg = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, rc)), tp, loc, 0, 1, 1, false, NO_EX).nil();
        if !sg.is_empty() {
            duel::special_summon(d, &sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
