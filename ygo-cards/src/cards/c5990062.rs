//! Reversal Quiz
use super::prelude::*;

pub const ID: u32 = 5990062;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    !card::is_able_to_grave_as_cost(d, c)
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let mut g = duel::get_field_group(d, a.tp, LOCATION_HAND | LOCATION_ONFIELD, 0);
    let c = effect::get_handler(d, a.e).nil();
    group::remove_card(&mut g, &CardSet::from_card(c));
    if a.chk == Some(0) {
        return !g.is_empty() && !group::is_exists(d, &g, &cfilter, 1, NO_EX);
    }
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_decktop_group(d, a.tp, 1);
    let Some(tc) = group::get_first(d, &g) else {
        return;
    };
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CARDTYPE as u64);
    let res = duel::select_option(d, a.tp, true, &[70, 71, 72]);
    duel::confirm_decktop(d, a.tp, 1);
    if (res == 0 && card::is_monster(d, tc)) || (res == 1 && card::is_spell(d, tc)) || (res == 2 && card::is_trap(d, tc)) {
        let lp1 = duel::get_lp(d, a.tp);
        let lp2 = duel::get_lp(d, 1 - a.tp);
        duel::set_lp(d, a.tp, lp2);
        duel::set_lp(d, 1 - a.tp, lp1);
    }
    duel::shuffle_deck(d, a.tp);
}
