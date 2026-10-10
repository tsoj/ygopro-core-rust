//! Stim-Pack
use super::prelude::*;

pub const ID: u32 = 83225447;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // atk up
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_value(d, e2, 700);
    card::register_effect(d, c, e2, false);
    // register
    let e3 = effect::create_effect(d, c);
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e3, EVENT_EQUIP);
    effect::set_operation(d, e3, Some(operation(regop)));
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_label_object(d, e3, Value::Effect(e2));
    card::register_effect(d, c, e3, false);
    // atkdown
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e4, EVENT_PHASE | PHASE_STANDBY);
    effect::set_range(d, e4, LOCATION_SZONE);
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    effect::set_label_object(d, e4, Value::Effect(e2));
    effect::set_condition(d, e4, Some(condition(atkcon)));
    effect::set_operation(d, e4, Some(operation(atkop)));
    card::register_effect(d, c, e4, false);
}
fn regop(d: &mut Duel, a: &EffArgs) {
    let eg = a.eg.clone().nil();
    let c = effect::get_handler(d, a.e).nil();
    if !group::is_contains(&eg, c) {
        return;
    }
    let pe = effect::get_label_object(d, a.e).as_effect().nil();
    effect::set_value(d, pe, 700);
}
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, a.tp)
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let pe = effect::get_label_object(d, a.e).as_effect().nil();
    let atk = effect::get_value(d, pe).int();
    effect::set_value(d, pe, atk - 200);
}
