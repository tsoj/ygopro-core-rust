//! Tribute Doll
use super::prelude::*;

pub const ID: u32 = 2903036;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon 1 level 7 monster, that can be normal summoned/set, from hand
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cfilter(d: &mut Duel, c: CardId, ft: i32, tp: u8) -> bool {
    ft > 0 || (card::is_controler(d, c, tp) && card::get_sequence(d, c) < 5)
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    if a.chk == Some(0) {
        return ft > -1 && duel::check_release_group_cost(d, a.tp, Some(&|d, c| cfilter(d, c, ft, a.tp)), 1, 1, false, None, NO_EX);
    }
    let g = duel::select_release_group_cost(d, a.tp, Some(&|d, c| cfilter(d, c, ft, a.tp)), 1, 1, false, None, NO_EX);
    duel::release(d, g, REASON_COST, None);
    true
}
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::get_level(d, c) == 7 && card::is_summonable_card(d, c) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        let tc = group::get_first(d, &g).nil();
        if duel::special_summon_step(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) {
            //Cannot attack
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            effect::set_description(d, e1, 3206);
            effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            let tc = group::get_first(d, &g).nil();
            card::register_effect(d, tc, e1, true);
        }
    }
    duel::special_summon_complete(d);
}
