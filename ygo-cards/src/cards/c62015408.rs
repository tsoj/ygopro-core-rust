//! Ghost Reaper & Winter Cherries
use super::prelude::*;

pub const ID: u32 = 62015408;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // remove
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_REMOVE);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER as u32));
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_cost(d, e1, Some(cost::self_discard()));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    duel::get_field_group_count(d, tp, LOCATION_MZONE, 0) < duel::get_field_group_count(d, tp, 0, LOCATION_MZONE)
        && duel::is_existing_matching_card(d, Some(&|_, _| true), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return !duel::is_player_affected_by_effect(d, tp, 30459350).is_some()
            && duel::get_field_group_count(d, tp, LOCATION_EXTRA, 0) > 0
            && duel::get_field_group_count(d, tp, 0, LOCATION_EXTRA) > 0;
    }
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - tp, LOCATION_EXTRA as i32);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    let sg = duel::select_matching_card(d, tp, Some(&|_, _| true), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    if !sg.is_empty() {
        duel::confirm_cards(d, 1 - tp, sg.clone());
        duel::break_effect(d);
        let g = duel::get_field_group(d, tp, 0, LOCATION_EXTRA);
        duel::confirm_cards(d, tp, g.clone());
        let codes = card::get_code(d, group::get_first(d, &sg).nil());
        let tg = group::filter(d, &g, &|d, c| card::is_code(d, c, &codes), NO_EX);
        if !tg.is_empty() {
            duel::remove(d, tg, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
        duel::shuffle_extra(d, 1 - tp);
    }
}
