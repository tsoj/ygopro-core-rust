//! Fiend Griefing
use super::prelude::*;

pub const ID: u32 = 60743819;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_DECKDES);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_monster(d, c) && card::is_able_to_deck(d, c)
}
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_race(d, c, RACE_FIEND) && card::is_able_to_grave(d, c, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - a.tp) && card::is_location(d, chkc, LOCATION_GRAVE) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, 0, LOCATION_GRAVE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, 0, LOCATION_GRAVE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_TODECK, g, 1, 0, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) && duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) != 0 {
        let g = duel::get_matching_group(d, Some(&tgfilter), tp, LOCATION_DECK, 0, NO_EX);
        if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
            duel::break_effect(d);
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
            let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
            duel::send_to_grave(d, &sg, REASON_EFFECT, None, None);
        }
    }
}
