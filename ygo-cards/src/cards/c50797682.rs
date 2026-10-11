//! Chronomaly Esperanza Glyph
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 50797682;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CHRONOMALY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_main_phase(d, None)
}
fn matfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, rank: u32) -> bool {
    card::is_set_card(d, c, &[SET_CHRONOMALY as u16]) && card::is_level(d, c, &[rank + 1]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn xyzfilter(d: &mut Duel, c: CardId, tp: u8, mg: &CardSet) -> bool {
    card::is_set_card(d, c, &[SET_CHRONOMALY as u16])
        && duel::get_location_count_from_ex(d, tp, Some(tp), mg.into(), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_xyz_summonable(d, c, None, Some(mg.clone()), 2, 2)
}
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    if card::is_original_type(d, c, TYPE_XYZ) && card::is_faceup(d, c) {
        let rank = card::get_rank(d, c);
        let mg = duel::get_matching_group(d, Some(&|d, x| matfilter(d, x, e, tp, rank)), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, NO_EX);
        return duel::is_existing_matching_card(d, Some(&|d, x| xyzfilter(d, x, tp, &mg)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    false
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_ONFIELD | LOCATION_GRAVE) && tgfilter(d, chkc, a.e, tp);
    }
    if a.chk == Some(0) {
        return duel::is_player_can_special_summon_count(d, tp, 2)
            && !duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some()
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 1
            && duel::is_existing_target(d, Some(&|d, x| tgfilter(d, x, a.e, tp)), tp, LOCATION_ONFIELD | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    duel::select_target(d, tp, Some(&|d, x| tgfilter(d, x, a.e, tp)), tp, LOCATION_ONFIELD | LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, 0, (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    (duel::is_existing_matching_card(d, Some(&|d, x| xyzfilter(d, x, tp, sg)), tp, LOCATION_EXTRA, 0, 1, NO_EX), false)
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 2 {
        return;
    }
    let tc = duel::first_target(d).nil();
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    let rank = card::get_rank(d, tc);
    let mg = duel::get_matching_group(d, Some(&|d, x| matfilter(d, x, a.e, tp, rank)), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, NO_EX);
    let sg = aux::select_unselect_group(d, &mg, Some(a.e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_SPSUMMON as u64), None, None, false)
        .unwrap_or_default();
    if sg.len() < 2 {
        return;
    }
    let order = group::iter(d, &sg);
    let tc1 = order[0];
    let tc2 = order[1];
    duel::special_summon_step(d, tc1, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    duel::special_summon_step(d, tc2, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    // Negate their effects
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_DISABLE);
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, tc1, e1, false);
    let e2 = effect::clone(d, e1);
    card::register_effect(d, tc2, e2, false);
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e3, EFFECT_DISABLE_EFFECT);
    effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, tc1, e3, false);
    let e4 = effect::clone(d, e3);
    card::register_effect(d, tc2, e4, false);
    duel::special_summon_complete(d);
    duel::break_effect(d);
    let xyzg = duel::get_matching_group(d, Some(&|d, x| xyzfilter(d, x, tp, &sg)), tp, LOCATION_EXTRA, 0, NO_EX);
    if !xyzg.is_empty() {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let sel = group::select(d, &xyzg, tp, 1, 1, false, NO_EX).nil();
        let xyz = group::get_first(d, &sel).nil();
        duel::xyz_summon(d, tp, xyz, None, Some(sg.clone()), 0, 0);
    }
}
