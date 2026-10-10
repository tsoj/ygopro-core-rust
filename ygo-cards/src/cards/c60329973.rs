//! Chaos Phantasm
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 60329973;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `s.tgfilter(c,e)`
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    if !(card::is_faceup(d, c) && card::has_level(d, c) && card::is_can_be_effect_target(d, c, Some(e))) {
        return false;
    }
    if card::is_attribute(d, c, ATTRIBUTE_LIGHT) {
        card::is_type(d, c, TYPE_TUNER)
    } else if card::is_attribute(d, c, ATTRIBUTE_DARK) {
        !card::is_type(d, c, TYPE_TUNER) && card::is_level_below(d, c, 8)
    } else {
        false
    }
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    let e = e.nil();
    let res = sg.len() == 2
        && group::is_exists(d, sg, &|d, c| card::is_attribute(d, c, ATTRIBUTE_LIGHT), 1, NO_EX)
        && group::is_exists(d, sg, &|d, c| card::is_attribute(d, c, ATTRIBUTE_DARK), 1, NO_EX)
        && {
            let lvl = group::get_sum(d, sg, &|d, c| card::get_level(d, c) as i64);
            duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp, lvl as u32)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
        };
    (res, false)
}
/// `s.spfilter(c,e,tp,lvl)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lvl: u32) -> bool {
    card::is_level(d, c, &[lvl])
        && card::is_type(d, c, TYPE_SYNCHRO)
        && card::is_attribute(d, c, ATTRIBUTE_LIGHT | ATTRIBUTE_DARK)
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,...,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let g = duel::get_matching_group(d, Some(&|d, c| tgfilter(d, c, a.e)), a.tp, LOCATION_REMOVED, 0, NO_EX);
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_REMOVED) && tgfilter(d, chkc, a.e);
    }
    if a.chk == Some(0) {
        return matches!(
            aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 0, a.tp, None, None, None, false),
            Err(true)
        );
    }
    let tg = aux::select_unselect_group(
        d,
        &g,
        Some(a.e),
        a.tp,
        Some(2),
        Some(2),
        Some(&rescon),
        1,
        a.tp,
        Some(HINTMSG_TOGRAVE as u64),
        None,
        None,
        false,
    )
    .unwrap_or_default();
    duel::set_target_card(d, tg.clone());
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(tg.clone()), tg.len() as u8, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let tg = duel::get_target_cards(d, a.e).nil();
    if tg.len() == 2 && duel::send_to_grave(d, &tg, REASON_EFFECT | REASON_RETURN, None, None) == 2 {
        let og = duel::get_operated_group(d);
        if group::filter_count(d, &og, &|d, c| card::is_location(d, c, LOCATION_GRAVE), NO_EX) != 2 {
            return;
        }
        let lvl = group::get_sum(d, &tg, &|d, c| card::get_level(d, c) as i64);
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp, lvl as u32)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX)
            .nil();
        if !g.is_empty() {
            duel::break_effect(d);
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
