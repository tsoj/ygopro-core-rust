//! Stardust Mirage
use super::prelude::*;

pub const ID: u32 = 13556444;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_hint_timing(d, e1, TIMING_END_PHASE, Some(TIMINGS_CHECK_MONSTER_E));
    effect::set_condition(d, e1, Some(condition(spcon)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_level_above(d, c, 8) && card::is_type(d, c, TYPE_SYNCHRO) && card::is_race(d, c, RACE_DRAGON) && card::is_faceup(d, c)
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, turn: i64) -> bool {
    card::get_turn_id(d, c) as i64 == turn
        && card::is_reason(d, c, REASON_DESTROY)
        && (card::is_reason(d, c, REASON_BATTLE) || (card::is_reason(d, c, REASON_EFFECT) && card::get_reason_player(d, c) == 1 - tp))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && {
            let turn = duel::get_turn_count(d, None) as i64;
            duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp, turn)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
        };
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    if ft == 0 {
        return;
    }
    let turn = duel::get_turn_count(d, None) as i64;
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, e, tp, turn)), tp, LOCATION_GRAVE, 0, NO_EX);
    if g.is_empty() {
        return;
    }
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let sg = group::select(d, &g, tp, ft as u16, ft as u16, false, NO_EX).nil();
    if !sg.is_empty() {
        duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
