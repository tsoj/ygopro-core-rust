//! Gold Sarcophagus
use super::prelude::*;

pub const ID: u32 = 75500286;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_REMOVE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn removable(d: &mut Duel, c: CardId) -> bool {
    // Card.IsAbleToRemove
    card::is_able_to_remove(d, c, None, None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&removable), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&removable), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let tg = match group::get_first(d, &g) {
        Some(tg) => tg,
        None => return,
    };
    duel::remove(d, tg, POS_FACEUP as u8, REASON_EFFECT, None, None);
    if !effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        return;
    }
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_range(d, e1, LOCATION_REMOVED);
    effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_STANDBY | RESET_SELF_TURN, 2);
    effect::set_condition(d, e1, Some(condition(thcon)));
    effect::set_operation(d, e1, Some(operation(thop)));
    effect::set_label(d, e1, &[0]);
    card::register_effect(d, tg, e1, false);
}
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp)
    duel::is_turn_player(d, a.tp)
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let ct = effect::get_label(d, a.e);
    let h = effect::get_handler(d, a.e).nil();
    card::set_turn_counter(d, h, (ct + 1) as u16);
    if ct == 1 {
        duel::send_to_hand(d, h, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - a.tp, h);
    } else {
        effect::set_label(d, a.e, &[1]);
    }
}
