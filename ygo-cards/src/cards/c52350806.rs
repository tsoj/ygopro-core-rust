//! Danger! Mothman!
//! Scripted by AlphaKretin
use super::prelude::*;

pub const ID: u32 = 52350806;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_HANDES + CATEGORY_DRAW);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // draw and discard
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_HANDES + CATEGORY_DRAW);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(drcon)));
    effect::set_target(d, e2, Some(cost_target(drtg)));
    effect::set_operation(d, e2, Some(operation(drop_op)));
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EVENT_REMOVE);
    card::register_effect(d, c, e3, false);
}
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return !card::is_public(d, c) && card::get_flag_effect(d, c, ID) == 0;
    }
    card::register_flag_effect(d, c, ID, RESET_CHAIN, 0, 1, 0, 0);
    true
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_code(d, c, &[ID]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(tp), Some(0xff))
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, a.tp, 1);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_field_group(d, a.tp, LOCATION_HAND, 0);
    if g.is_empty() {
        return;
    }
    duel::shuffle_hand(d, a.tp);
    duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_DISCARD as u64);
    let tc = group::random_select(d, &g, 1 - a.tp, 1);
    duel::break_effect(d);
    duel::send_to_grave(d, tc.clone(), REASON_EFFECT | REASON_DISCARD, None, None);
    if !duel::is_player_can_special_summon(d, a.tp, None) || duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 == 0 {
        return;
    }
    let first = group::get_first(d, &tc).nil();
    if !card::is_code(d, first, &[ID]) {
        duel::break_effect(d);
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        let sc = duel::select_matching_card(d, a.tp, Some(&|d, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        // Lua: `#sc>0 and Duel.SpecialSummon(...)`: any returned number is true
        if !sc.is_empty() {
            duel::special_summon(d, sc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
            duel::draw(d, a.tp, 1, REASON_EFFECT);
        }
    }
}
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::get_previous_location(d, c) == LOCATION_HAND && (a.r & REASON_DISCARD) != 0
}
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1) && duel::is_player_can_draw(d, 1 - a.tp, 1);
    }
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, PLAYER_ALL as u8, 1);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, PLAYER_ALL as u8, 1);
    true
}
fn drop_op(d: &mut Duel, a: &EffArgs) {
    let h1 = duel::draw(d, a.tp, 1, REASON_EFFECT);
    let h2 = duel::draw(d, 1 - a.tp, 1, REASON_EFFECT);
    if h1 > 0 || h2 > 0 {
        duel::break_effect(d);
    }
    if h1 > 0 {
        duel::shuffle_hand(d, a.tp);
        duel::discard_hand(d, a.tp, Some(&|_d, _c| true), 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
    }
    if h2 > 0 {
        duel::shuffle_hand(d, 1 - a.tp);
        duel::discard_hand(d, 1 - a.tp, Some(&|_d, _c| true), 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
    }
}
