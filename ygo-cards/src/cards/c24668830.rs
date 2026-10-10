//! Germ Infection
use super::prelude::*;

pub const ID: u32 = 24668830;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,aux.NOT(aux.FilterBoolFunction(Card.IsRace,RACE_MACHINE)))
    aux::add_equip_procedure(
        d,
        c,
        None,
        Some(aux::equip_filter_fn(|d, c, _e, _tp| !card::is_race(d, c, RACE_MACHINE))),
        None,
        None,
        None,
        None,
        None,
        None,
    );
    // atkdown
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_value(d, e2, 0);
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
/// `s.regop(e,...)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // if not eg:IsContains(e:GetHandler()) then return end
    let eg = a.eg.clone().nil();
    let h = effect::get_handler(d, a.e).nil();
    if !group::is_contains(&eg, h) {
        return;
    }
    // local pe=e:GetLabelObject()
    // pe:SetValue(0)
    let pe = effect::get_label_object(d, a.e).as_effect().nil();
    effect::set_value(d, pe, 0);
}
/// `s.atkcon(e,tp,...)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp)
    duel::is_turn_player(d, a.tp)
}
/// `s.atkop(e,...)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local pe=e:GetLabelObject()
    let pe = effect::get_label_object(d, a.e).as_effect().nil();
    // local atk=pe:GetValue()
    let atk = effect::get_value(d, pe).int();
    // pe:SetValue(atk-300)
    effect::set_value(d, pe, atk - 300);
}
