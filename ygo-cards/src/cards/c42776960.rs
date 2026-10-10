//! Relay Soul
use super::prelude::*;

pub const ID: u32 = 42776960;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    if let Some(tc) = tc {
        if duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
            let handler = effect::get_handler(d, e).nil();
            let e1 = effect::create_effect(d, handler);
            effect::set_type(d, e1, EFFECT_TYPE_FIELD);
            effect::set_code(d, e1, EFFECT_CHANGE_DAMAGE);
            effect::set_range(d, e1, LOCATION_MZONE);
            effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CANNOT_DISABLE, 0);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            effect::set_target_range(d, e1, 1, 0);
            effect::set_condition(d, e1, Some(condition(con)));
            effect::set_value(d, e1, 0);
            card::register_effect(d, tc, e1, true);
            let e2 = effect::clone(d, e1);
            effect::set_code(d, e2, EFFECT_NO_EFFECT_DAMAGE);
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, tc, e2, true);
            let e3 = effect::create_effect(d, handler);
            effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
            effect::set_code(d, e3, EVENT_LEAVE_FIELD);
            effect::set_label(d, e3, &[(1 - tp) as i64]);
            effect::set_operation(d, e3, Some(operation(leaveop)));
            effect::set_reset(d, e3, RESET_EVENT | RESET_OVERLAY | RESET_MSCHANGE | RESET_TURN_SET, 1);
            card::register_effect(d, tc, e3, true);
        }
    }
    duel::special_summon_complete(d);
}
fn con(d: &mut Duel, a: &EffArgs) -> bool {
    effect::get_handler_player(d, a.e) == effect::get_owner_player(d, a.e)
}
fn leaveop(d: &mut Duel, a: &EffArgs) {
    let label = effect::get_label(d, a.e);
    duel::win(d, label as u8, WIN_REASON_RELAY_SOUL as u32);
}
