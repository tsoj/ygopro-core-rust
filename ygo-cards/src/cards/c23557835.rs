//! Dimension Fusion
use super::prelude::*;

pub const ID: u32 = 23557835;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(2000))));
    effect::set_target(d, e1, Some(cost_target(tg)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_faceup(d, c) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn tg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return (duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_REMOVED, 0, 1, NO_EX))
            || (duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
                && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.e, 1 - a.tp)), a.tp, 0, LOCATION_REMOVED, 1, NO_EX));
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 2, LOCATION_REMOVED as i32);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let mut count = 0;
    let mut ft1 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    if ft1 > 0 {
        if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
            ft1 = 1;
        }
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let g = duel::select_matching_card(d, tp, Some(&|d, c| filter(d, c, a.e, tp)), tp, LOCATION_REMOVED, 0, ft1 as u16, ft1 as u16, false, NO_EX).nil();
        if !g.is_empty() {
            let _ = group::get_first(d, &g);
            for tc in group::iter(d, &g) {
                duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
                count += 1;
            }
        }
    }
    let mut ft2 = duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0;
    if ft2 > 0 {
        if duel::is_player_affected_by_effect(d, 1 - tp, CARD_BLUEEYES_SPIRIT).is_some() {
            ft2 = 1;
        }
        duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_SPSUMMON as u64);
        let g = duel::select_matching_card(d, 1 - tp, Some(&|d, c| filter(d, c, a.e, 1 - tp)), tp, 0, LOCATION_REMOVED, ft2 as u16, ft2 as u16, false, NO_EX).nil();
        if !g.is_empty() {
            let _ = group::get_first(d, &g);
            for tc in group::iter(d, &g) {
                duel::special_summon_step(d, tc, 0, 1 - tp, 1 - tp, false, false, POS_FACEUP as u8, None);
                count += 1;
            }
        }
    }
    if count > 0 {
        duel::special_summon_complete(d);
    }
}
