//! Painful Choice
use super::prelude::*;

pub const ID: u32 = 74191942;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND | CATEGORY_SEARCH | CATEGORY_TOGRAVE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_able_to_hand(d, c, None) && card::is_able_to_grave(d, c, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_DECK, 0, 5, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 4, a.tp, LOCATION_DECK as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    if duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) < 5 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONFIRM as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&filter), a.tp, LOCATION_DECK, 0, 5, 5, false, NO_EX).nil();
    if g.len() != 5 {
        return;
    }
    let opp = 1 - a.tp;
    duel::confirm_cards(d, opp, g.clone());
    duel::hint(d, HINT_SELECTMSG as u8, opp, aux::stringid(ID, 1));
    let sg = group::select(d, &g, opp, 1, 1, false, NO_EX).nil();
    if duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None) > 0 {
        let mut rest = g.clone();
        group::remove_card(&mut rest, &sg);
        duel::send_to_grave(d, rest, REASON_EFFECT, None, None);
    }
}
