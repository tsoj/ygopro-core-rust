//! Mimighoul Room
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 59293853;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MIMIGHOUL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "Mimighoul" monster
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON | CATEGORY_POSITION | CATEGORY_SET);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Return any number of "Mimighoul" cards you control to the hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOHAND);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, f1: bool, f2: bool) -> bool {
    if !card::is_set_card(d, c, &[SET_MIMIGHOUL as u16]) {
        return false;
    }
    (f1 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None))
        || (f2 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), Some(1 - tp), None))
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chk == Some(0) {
        let f1 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
        let f2 = duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, Some(tp), None, None).0 > 0;
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp, f1, f2)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_POSITION, None, 1, PLAYER_EITHER as u8, POS_FACEDOWN_DEFENSE as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let f1 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    let f2 = duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, Some(tp), None, None).0 > 0;
    if !(f1 || f2) {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, f1, f2)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let Some(tc) = group::get_first(d, &g) else {
        return;
    };
    let b1 = f1 && card::is_can_be_special_summoned(d, tc, e, 0, tp, false, false, None, None, None);
    let b2 = f2 && card::is_can_be_special_summoned(d, tc, e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), Some(1 - tp), None);
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]);
    let target_player = if op == Some(1) { tp } else { 1 - tp };
    let summon_pos = if op == Some(1) { POS_FACEUP } else { POS_FACEDOWN_DEFENSE };
    if duel::special_summon(d, tc, 0, tp, target_player, false, false, summon_pos as u8, None) == 0 {
        return;
    }
    if card::is_facedown(d, tc) {
        duel::confirm_cards(d, tp, tc);
    }
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_can_turn_set(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    if g.is_empty() || !duel::select_yes_no(d, tp, aux::stringid(ID, 4)) {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_POSCHANGE as u64);
    let pg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
    if let Some(pc) = group::get_first(d, &pg) {
        duel::hint_selection(d, pc, true);
        duel::break_effect(d);
        duel::change_position(d, pc, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
    }
}
fn thfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_MIMIGHOUL as u16])
        && card::is_able_to_hand(d, c, None)
        && card::is_can_be_effect_target(d, c, Some(e))
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_ONFIELD) && thfilter(d, chkc, e);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| thfilter(d, c, e)), tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    let g = duel::get_matching_group(d, Some(&|d, c| thfilter(d, c, e)), tp, LOCATION_ONFIELD, 0, NO_EX);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    let tg = group::select(d, &g, tp, 1, g.len() as u16, false, NO_EX).nil();
    duel::set_target_card(d, tg.clone());
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(tg.clone()), tg.len() as u8, tp, 0);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tg = duel::get_target_cards(d, a.e).nil();
    if !tg.is_empty() {
        duel::send_to_hand(d, tg, None, REASON_EFFECT, None);
    }
}
