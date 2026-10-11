//! Bright Future
use super::prelude::*;

pub const ID: u32 = 24707869;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_DRAW);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_race(d, c, RACE_PSYCHIC)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_REMOVED) && card::is_controler(d, chkc, tp) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, 1)
            && duel::is_existing_target(d, Some(&filter), tp, LOCATION_REMOVED, 0, 2, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    let _ = duel::select_target(d, tp, Some(&filter), tp, LOCATION_REMOVED, 0, 2, 2, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let tg = duel::get_target_cards(d, a.e).nil();
    if tg.len() != 2 {
        return;
    }
    duel::send_to_grave(d, tg, REASON_EFFECT | REASON_RETURN, None, None);
    duel::break_effect(d);
    duel::draw(d, tp, 1, REASON_EFFECT);
}
