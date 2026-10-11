//! Icejade Erosion
use super::prelude::*;

pub const ID: u32 = 53742162;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ICEJADE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e0, false);
    // Negate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY | CATEGORY_DISABLE);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e1, LOCATION_SZONE);
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER));
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(negtg)));
    effect::set_operation(d, e1, Some(operation(negop)));
    card::register_effect(d, c, e1, false);
    // Add to hand or Special Summon
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SEARCH | CATEGORY_TOHAND | CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD | EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_LEAVE_FIELD);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(thcon)));
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
}
fn faceup_icejade(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_ICEJADE as u16])
}
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - a.tp) && card::is_on_field(d, chkc) && card::is_negatable(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&faceup_icejade), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&card::is_negatable), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_NEGATE as u64);
    let g = duel::select_target(d, a.tp, Some(&card::is_negatable), a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    let dg = duel::get_matching_group(d, Some(&faceup_icejade), a.tp, LOCATION_MZONE, 0, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg), 1, a.tp, LOCATION_MZONE as i32);
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, g, 1, 1 - a.tp, LOCATION_ONFIELD as i32);
    true
}
fn negop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    let dg = duel::select_matching_card(d, a.tp, Some(&faceup_icejade), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    if dg.len() == 0 || duel::destroy(d, dg, REASON_EFFECT, None, None) <= 0 {
        return;
    }
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e)
            && card::is_faceup(d, tc)
            && card::is_controler(d, tc, 1 - a.tp)
            && !card::is_disabled(d, tc)
        {
            // Negate effects
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
    }
}
fn thconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_previous_position(d, c, POS_FACEUP)
        && card::is_previous_controler(d, c, tp)
        && !card::is_reason(d, c, REASON_DESTROY)
        && (card::get_previous_attribute_on_field(d, c) & ATTRIBUTE_WATER) == ATTRIBUTE_WATER
}
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    a.rp == 1 - a.tp && {
        let eg = a.eg.clone().nil();
        group::is_exists(d, &eg, &|d, c| thconfilter(d, c, a.tp), 1, NO_EX)
    }
}
fn thfilter(d: &mut Duel, c: CardId, ft: i32, e: EffectId, tp: u8) -> bool {
    card::is_monster(d, c)
        && card::is_set_card(d, c, &[SET_ICEJADE as u16])
        && (card::is_able_to_hand(d, c, None)
            || (ft > 0 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
        return duel::is_existing_matching_card(d, Some(&|d, c| thfilter(d, c, ft, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let sc = duel::select_matching_card(d, a.tp, Some(&|d, c| thfilter(d, c, ft, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = match group::get_first(d, &sc) {
        Some(sc) => sc,
        None => return,
    };
    aux::to_hand_or_else(
        d,
        sc,
        a.tp,
        Some(&|d, x| ft > 0 && card::is_can_be_special_summoned(d, x, a.e, 0, a.tp, false, false, None, None, None)),
        Some(&|d, g: &CardSet| duel::special_summon(d, g.clone(), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None)),
        Some(aux::stringid(ID, 2)),
    );
}
