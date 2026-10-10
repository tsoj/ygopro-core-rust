//! Photon Sabre Tiger
use super::prelude::*;

pub const ID: u32 = 80495985;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // search
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_TRIGGER_O + EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_target(d, e1, Some(cost_target(thtg)));
    effect::set_operation(d, e1, Some(operation(thop)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EVENT_FLIP_SUMMON_SUCCESS);
    card::register_effect(d, c, e2, false);
    // atkdown
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    effect::set_condition(d, e3, Some(condition(atcon)));
    effect::set_value(d, e3, -800);
    card::register_effect(d, c, e3, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::get_first_matching_card(d, Some(&filter), a.tp, LOCATION_DECK, 0, NO_EX);
    if let Some(tc) = tc {
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - a.tp, tc);
    }
}
fn atfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_code(d, c, &[ID])
}
fn atcon(d: &mut Duel, a: &EffArgs) -> bool {
    let handler = effect::get_handler(d, a.e).nil();
    let tp = effect::get_handler_player(d, a.e);
    !duel::is_existing_matching_card(d, Some(&atfilter), tp, LOCATION_ONFIELD, 0, 1, handler.into())
}
