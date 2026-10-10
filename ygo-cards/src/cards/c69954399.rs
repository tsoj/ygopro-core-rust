//! Ekibyo Drakmord
use super::prelude::*;

pub const ID: u32 = 69954399;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,nil,nil,nil,nil,s.eqpop1,nil)
    aux::add_equip_procedure(d, c, None, None, None, None, None, Some(operation(eqpop1)), None, None);
    // Cannot attack
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e2, EFFECT_CANNOT_ATTACK);
    card::register_effect(d, c, e2, false);
    // Count turns
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_condition(d, e3, Some(condition(turncon)));
    effect::set_operation(d, e3, Some(operation(turnop)));
    card::register_effect(d, c, e3, false);
    // Destroy monster
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 0));
    effect::set_category(d, e4, CATEGORY_DESTROY);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e4, EVENT_PHASE + PHASE_END);
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    effect::set_range(d, e4, LOCATION_SZONE);
    effect::set_condition(d, e4, Some(condition(descon)));
    effect::set_target(d, e4, Some(cost_target(destg)));
    effect::set_operation(d, e4, Some(operation(desop)));
    card::register_effect(d, c, e4, false);
    // return itself to the hand
    let e5 = effect::create_effect(d, c);
    effect::set_description(d, e5, aux::stringid(ID, 1));
    effect::set_category(d, e5, CATEGORY_TOHAND);
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e5, EVENT_CUSTOM + ID + 1);
    effect::set_target(d, e5, Some(cost_target(rettg)));
    effect::set_operation(d, e5, Some(operation(retop)));
    card::register_effect(d, c, e5, false);
}
fn eqpop1(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    card::set_turn_counter(d, h, 0);
}
fn turncon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    let ec = card::get_equip_target(d, h).nil();
    card::get_controler(d, ec) == duel::get_turn_player(d)
}
fn turnop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let ct = card::get_turn_counter(d, c);
    card::set_turn_counter(d, c, (ct + 1) as u16);
    card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END, 0, 0, 0, 0);
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::get_turn_counter(d, c) == 2 && card::get_flag_effect(d, c, ID) > 0
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let h = effect::get_handler(d, a.e).nil();
    let ec = card::get_equip_target(d, h).nil();
    card::create_effect_relation(d, ec, a.e);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(ec)), 1, 0, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if let Some(ec) = card::get_equip_target(d, c) {
        if card::is_relate_to_effect(d, ec, a.e) && duel::destroy(d, ec, REASON_EFFECT, None, None) != 0 {
            duel::raise_single_event(d, c, EVENT_CUSTOM + ID + 1, Some(a.e), 0, 0, 0, 0);
        }
    }
}
fn rettg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let c = effect::get_handler(d, a.e).nil();
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
