//! Over Limit
use super::prelude::*;

pub const ID: u32 = 23282832;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(500))));
    effect::set_target(d, e1, Some(cost_target(tg)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tid: i32) -> bool {
    card::get_turn_id(d, c) as i32 == tid
        && card::is_reason(d, c, REASON_BATTLE)
        && card::is_type(d, c, TYPE_NORMAL)
        && card::is_attack_below(d, c, 1000)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn tg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        let tid = duel::get_turn_count(d, None);
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.e, tp, tid)), tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, PLAYER_ALL as u8, LOCATION_GRAVE as i32);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    if ft <= 0 {
        return;
    }
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let tid = duel::get_turn_count(d, None);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| filter(d, c, a.e, tp, tid)), tp, LOCATION_GRAVE, LOCATION_GRAVE, ft as u16, ft as u16, false, NO_EX).nil();
    if !g.is_empty() {
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
