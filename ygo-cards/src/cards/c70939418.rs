//! Speedroid Gum Prize
use super::prelude::*;

pub const ID: u32 = 70939418;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SPEEDROID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL + EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_DAMAGE);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_condition(d, e1, Some(condition(spcon)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // synchro summon
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_hint_timing(d, e2, TIMING_BATTLE_STEP_END | TIMING_BATTLE_END, None);
    effect::set_condition(d, e2, Some(condition(sccon)));
    effect::set_target(d, e2, Some(cost_target(sctg)));
    effect::set_operation(d, e2, Some(operation(scop)));
    card::register_effect(d, c, e2, false);
    // deckdes
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 2));
    effect::set_category(d, e3, CATEGORY_ATKCHANGE);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e3, EVENT_BE_MATERIAL);
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    effect::set_condition(d, e3, Some(condition(ddcon)));
    effect::set_target(d, e3, Some(cost_target(ddtg)));
    effect::set_operation(d, e3, Some(operation(ddop)));
    card::register_effect(d, c, e3, false);
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_battle_phase(d, None) && a.ep == a.tp && (a.r & (REASON_BATTLE | REASON_EFFECT)) != 0
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    if duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) != 0 {
        let reset = RESET_EVENT | (RESETS_STANDARD & !(RESET_TEMP_REMOVE | RESET_TURN_SET)) | RESET_PHASE | PHASE_BATTLE;
        card::register_flag_effect(d, c, ID, reset, 0, 1, 0, 0);
    }
}
fn sccon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, c, ID) != 0
}
fn mfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_WIND)
}
fn sctg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        let mg = duel::get_matching_group(d, Some(&mfilter), a.tp, LOCATION_MZONE, 0, NO_EX);
        return !card::is_status(d, c, STATUS_CHAINING)
            && duel::is_existing_matching_card(d, Some(&|d, x| card::is_synchro_summonable(d, x, Some(CardSet::from_card(c)), Some(mg.clone()), 0, 0)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
fn scop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::get_controler(d, c) != a.tp || !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let mg = duel::get_matching_group(d, Some(&mfilter), a.tp, LOCATION_MZONE, 0, NO_EX);
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_synchro_summonable(d, x, Some(CardSet::from_card(c)), Some(mg.clone()), 0, 0)), a.tp, LOCATION_EXTRA, 0, NO_EX);
    if !g.is_empty() {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &sg).nil();
        duel::synchro_summon(d, a.tp, tc, Some(CardSet::from_card(c)), Some(mg), 0, 0);
    }
}
fn ddcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_location(d, c, LOCATION_GRAVE) && a.r == REASON_SYNCHRO
}
fn ddtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck(d, a.tp, 1);
    }
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, a.tp, 1);
    true
}
fn ddop(d: &mut Duel, a: &EffArgs) {
    if duel::discard_deck(d, a.tp, 1, REASON_EFFECT) == 0 {
        return;
    }
    let og = duel::get_operated_group(d);
    let tc = group::get_first(d, &og);
    let c = effect::get_handler(d, a.e).nil();
    let sync = card::get_reason_card(d, c);
    if let Some(tc) = tc {
        if card::is_set_card(d, tc, &[SET_SPEEDROID as u16]) && card::is_monster(d, tc) && card::is_location(d, tc, LOCATION_GRAVE) {
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            effect::set_value(d, e1, 1000);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, sync.nil(), e1, false);
        }
    }
}
