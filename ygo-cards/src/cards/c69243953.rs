//! Butterfly Dagger - Elma
use super::prelude::*;

pub const ID: u32 = 69243953;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // Atk
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_value(d, e2, 300);
    card::register_effect(d, c, e2, false);
    // tohand
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 0));
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_category(d, e4, CATEGORY_TOHAND);
    effect::set_code(d, e4, EVENT_LEAVE_FIELD);
    effect::set_condition(d, e4, Some(condition(retcon)));
    effect::set_target(d, e4, Some(cost_target(rettg)));
    effect::set_operation(d, e4, Some(operation(retop)));
    card::register_effect(d, c, e4, false);
}
fn retcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_location(d, c, LOCATION_GRAVE) && card::is_reason(d, c, REASON_DESTROY) && card::get_equip_target(d, c).is_some()
}
fn rettg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None);
    }
    card::create_effect_relation(d, c, a.e);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn retop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::send_to_hand(d, c, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - a.tp, c);
    }
}
