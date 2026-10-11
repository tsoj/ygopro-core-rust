//! Lancea, Ancestral Dragon of the Ice Mountain
use super::prelude::*;

pub const ID: u32 = 96402918;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ICE_BARRIER], ..ScriptInfo::DEFAULT };

const LOCATION_HAND_DECK_EXTRA_GRAVE: u32 = LOCATION_HAND | LOCATION_DECK | LOCATION_EXTRA | LOCATION_GRAVE;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure
    synchro::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_attribute_ctx(d, c, ATTRIBUTE_WATER, sc, st, Some(tp)))),
        1,
        1,
        Some(synchro::non_tuner(None)),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // Special Summon 1 "Ice Barrier" monster from your hand, Deck, Extra Deck, or GY
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON | CATEGORY_POSITION);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD | EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 2, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(cond1)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "Ice Barrier" Synchro Monster from your Extra Deck
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE | EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_LEAVE_FIELD);
    effect::set_condition(d, e2, Some(condition(exspcon)));
    effect::set_target(d, e2, Some(cost_target(exsptg)));
    effect::set_operation(d, e2, Some(operation(exspop)));
    card::register_effect(d, c, e2, false);
}
fn cond1(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| card::is_summon_player(d, c, 1 - a.tp), 1, NO_EX)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    if !(card::is_set_card(d, c, &[SET_ICE_BARRIER as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)) {
        return false;
    }
    if card::is_location(d, c, LOCATION_EXTRA) {
        duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
    } else {
        duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
    }
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return !card::has_flag_effect(d, c, ID, None)
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_HAND_DECK_EXTRA_GRAVE, 0, 1, NO_EX);
    }
    card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD | RESET_CHAIN, 0, 1, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HAND_DECK_EXTRA_GRAVE as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_POSITION, None, 1, 1 - a.tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let f = aux::necro_valley_filter(|d: &mut Duel, x: CardId| spfilter(d, x, a.e, a.tp));
    let g = duel::select_matching_card(d, a.tp, Some(&f), a.tp, LOCATION_HAND_DECK_EXTRA_GRAVE, 0, 1, 1, false, NO_EX).nil();
    if g.len() == 0 || duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) == 0 {
        return;
    }
    if duel::is_existing_matching_card(
        d,
        Some(&|d, x| card::is_attack_pos(d, x) && card::is_can_change_position(d, x)),
        a.tp,
        0,
        LOCATION_MZONE,
        1,
        NO_EX,
    ) && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2))
    {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_POSCHANGE as u64);
        let posg = duel::select_matching_card(
            d,
            a.tp,
            Some(&|d, x| card::is_attack_pos(d, x) && card::is_can_change_position(d, x)),
            a.tp,
            0,
            LOCATION_MZONE,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        if posg.len() == 0 {
            return;
        }
        duel::hint_selection(d, posg.clone(), true);
        duel::break_effect(d);
        duel::change_position(d, posg, POS_FACEUP_DEFENSE as u8, Some(POS_FACEDOWN_DEFENSE as u8), None, None, false);
    }
}
fn exspcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    a.rp == 1 - a.tp
        && card::is_previous_position(d, c, POS_FACEUP)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_controler(d, c, a.tp)
        && card::is_synchro_summoned(d, c)
}
fn exspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_ICE_BARRIER as u16])
        && card::is_type(d, c, TYPE_SYNCHRO)
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_SYNCHRO, tp, false, false, None, None, None)
}
fn exsptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| exspfilter(d, x, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
fn exspop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| exspfilter(d, x, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    if let Some(tc) = group::get_first(d, &g) {
        if duel::special_summon(d, tc, SUMMON_TYPE_SYNCHRO, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
            card::complete_procedure(d, tc);
        }
    }
}
