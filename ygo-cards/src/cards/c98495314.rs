//! Sword of Deep-Seated
use super::prelude::*;

pub const ID: u32 = 98495314;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // Atk up
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_value(d, e2, 500);
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    card::register_effect(d, c, e3, false);
    // to deck
    let e5 = effect::create_effect(d, c);
    effect::set_description(d, e5, aux::stringid(ID, 0));
    effect::set_category(d, e5, CATEGORY_TODECK);
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e5, EVENT_TO_GRAVE);
    effect::set_target(d, e5, Some(cost_target(tdtg)));
    effect::set_operation(d, e5, Some(operation(tdop)));
    card::register_effect(d, c, e5, false);
}
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return card::is_relate_to_effect(d, c, a.e);
    }
    let targets = effect::get_handler(d, a.e).map(CardSet::from_card);
    duel::set_operation_info(d, 0, CATEGORY_TODECK, targets, 1, 0, 0);
    true
}
fn tdop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::send_to_deck(d, c, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
    }
}
