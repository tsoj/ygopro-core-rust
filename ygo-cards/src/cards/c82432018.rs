//! Mask of Brutality
use super::prelude::*;

pub const ID: u32 = 82432018;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c)
    aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // Atk,def
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_value(d, e2, 1000);
    card::register_effect(d, c, e2, false);
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    effect::set_value(d, e3, -1000);
    card::register_effect(d, c, e3, false);
    // cost
    let e5 = effect::create_effect(d, c);
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e5, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e5, EVENT_PHASE | PHASE_STANDBY);
    effect::set_range(d, e5, LOCATION_SZONE);
    effect::set_count_limit(d, e5, 1, 0, 0, 0);
    effect::set_condition(d, e5, Some(condition(mtcon)));
    effect::set_operation(d, e5, Some(operation(mtop)));
    card::register_effect(d, c, e5, false);
}
fn mtcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, a.tp)
}
fn mtop(d: &mut Duel, a: &EffArgs) {
    if duel::check_lp_cost(d, a.tp, 1000) && duel::select_yes_no(d, a.tp, aux::stringid(ID, 0)) {
        duel::pay_lp_cost(d, a.tp, 1000);
    } else {
        let c = effect::get_handler(d, a.e).nil();
        duel::destroy(d, c, REASON_COST, None, None);
    }
}
