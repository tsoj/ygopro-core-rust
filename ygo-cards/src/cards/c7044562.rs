//! Card of the Soul
use super::prelude::*;

pub const ID: u32 = 7044562;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) > 0;
    }
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn filter(d: &mut Duel, c: CardId, lp: i32) -> bool {
    card::get_attack(d, c) >= 0 && card::get_defense(d, c) >= 0 && card::get_attack(d, c) + card::get_defense(d, c) == lp && card::is_able_to_hand(d, c, None)
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let g = duel::get_field_group(d, tp, LOCATION_DECK, 0);
    if g.is_empty() {
        return;
    }
    duel::confirm_cards(d, tp, &g);
    let lp = duel::get_lp(d, tp);
    if group::is_exists(d, &g, &|d, c| filter(d, c, lp), 1, NO_EX) &&duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        let sg = group::filter_select(d, &g, tp, &|d, c| filter(d, c, lp), 1, 1, false, NO_EX).nil();
        duel::send_to_hand(d, &sg, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, &sg);
        duel::shuffle_hand(d, tp);
    }
    duel::shuffle_deck(d, tp);
}
