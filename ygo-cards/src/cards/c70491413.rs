//! Surprise Chain
use super::prelude::*;

pub const ID: u32 = 70491413;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::get_current_chain(d, true) > 0 && duel::check_chain_uniqueness(d)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let cl = duel::get_current_chain(d, false);
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, tp, LOCATION_DECK, 0) >= cl
            && (cl < 3 || duel::is_player_can_discard_deck(d, tp, 1))
            && (cl < 4 || duel::is_player_can_draw(d, tp, 1));
    }
    if cl >= 3 {
        duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
    }
    if cl >= 4 {
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    }
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let mut break_chk = false;
    let cl = duel::get_current_chain(d, false);
    if cl >= 2 {
        break_chk = true;
        duel::confirm_decktop(d, tp, cl as u32);
        duel::sort_decktop(d, tp, tp, cl as u16);
    }
    if cl >= 3 && duel::is_player_can_discard_deck(d, tp, 1) {
        if break_chk {
            duel::break_effect(d);
        }
        break_chk = true;
        duel::discard_deck(d, tp, 1, REASON_EFFECT);
    }
    if cl >= 4 {
        if break_chk {
            duel::break_effect(d);
        }
        duel::draw(d, tp, 1, REASON_EFFECT);
    }
}
