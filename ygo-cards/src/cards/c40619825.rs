//! Axe of Despair
use super::prelude::*;

pub const ID: u32 = 40619825;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c)
    aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // Atk up
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_value(d, e1, 1000);
    card::register_effect(d, c, e1, false);
    // to deck
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_TODECK);
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_condition(d, e2, Some(condition(tdcon)));
    effect::set_cost(d, e2, Some(cost_target(tdcost)));
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    effect::set_operation(d, e2, Some(operation(tdop)));
    card::register_effect(d, c, e2, false);
}
fn tdcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, c, LOCATION_ONFIELD)
}
fn tdcost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, None, 1, 1, false, None, NO_EX);
    }
    let g = duel::select_release_group_cost(d, a.tp, None, 1, 1, false, None, NO_EX);
    duel::release(d, g, REASON_COST, None);
    true
}
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_deck(d, c);
    }
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn tdop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::send_to_deck(d, c, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
    }
}
