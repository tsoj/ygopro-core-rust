//! Tri-Brigade Airborne Assault
use super::prelude::*;

pub const ID: u32 = 51097887;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 1));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, atk: i32, race: u64) -> bool {
    card::is_race(d, c, RACES_BEAST_BWARRIOR_WINGB as u64)
        && card::is_attack_below(d, c, atk)
        && !card::is_race(d, c, race)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
fn tgtfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_faceup(d, c)
        && card::is_race(d, c, RACES_BEAST_BWARRIOR_WINGB as u64)
        && {
            let atk = card::get_attack(d, c);
            let race = card::get_race(d, c);
            duel::is_existing_matching_card(d, Some(&|d, sc| spfilter(d, sc, e, tp, atk, race)), tp, LOCATION_DECK, 0, 1, NO_EX)
        }
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && tgtfilter(d, chkc, a.e, a.tp);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| tgtfilter(d, c, a.e, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    let _g = duel::select_target(d, a.tp, Some(&|d, c| tgtfilter(d, c, a.e, a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 0, LOCATION_DECK as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_FIELD);
    effect::set_property(d, e0, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e0, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_reset(d, e0, RESET_PHASE | PHASE_END, 1);
    effect::set_target_range(d, e0, 1, 0);
    effect::set_target(d, e0, Some(effect_card_fn(splimit)));
    duel::register_effect(d, e0, a.tp);
    aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
    let tc = duel::first_target(d);
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < 1 || !card::is_relate_to_effect(d, tc.nil(), a.e) {
        return;
    }
    let tc = tc.nil();
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let atk = card::get_attack(d, tc);
    let race = card::get_race(d, tc);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, sc| spfilter(d, sc, a.e, a.tp, atk, race)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX);
    let g = g.nil();
    if let Some(sc) = group::get_first(d, &g) {
        if duel::special_summon_step(d, sc, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None) {
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_DISABLE);
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            card::register_effect(d, sc, e1, true);
            let e2 = effect::create_effect(d, c);
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
            effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
            card::register_effect(d, sc, e2, true);
        }
    }
    duel::special_summon_complete(d);
}
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_location(d, c, LOCATION_EXTRA) && !card::is_type(d, c, TYPE_LINK)
}
