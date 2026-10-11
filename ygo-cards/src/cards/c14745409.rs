//! Noble Arms - Gallatin
use super::prelude::*;

pub const ID: u32 = 14745409;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_NOBLE_KNIGHT],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    aux::add_equip_procedure(
        d,
        c,
        None,
        Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_race(d, c, RACE_WARRIOR))),
        None,
        None,
        None,
        None,
        None,
        None,
    );
    // Atk up
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_value(d, e2, 1000);
    card::register_effect(d, c, e2, false);
    // register
    let e3 = effect::create_effect(d, c);
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e3, EVENT_EQUIP);
    effect::set_operation(d, e3, Some(operation(regop)));
    effect::set_label_object(d, e3, Value::Effect(e2));
    effect::set_range(d, e3, LOCATION_SZONE);
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
    // equip
    let e5 = effect::create_effect(d, c);
    effect::set_description(d, e5, aux::stringid(ID, 0));
    effect::set_category(d, e5, CATEGORY_EQUIP);
    effect::set_property(d, e5, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e5, EVENT_TO_GRAVE);
    effect::set_count_limit(d, e5, 1, ID, 0, 0);
    effect::set_condition(d, e5, Some(condition(eqcon)));
    effect::set_target(d, e5, Some(cost_target(eqtg)));
    effect::set_operation(d, e5, Some(operation(eqop)));
    card::register_effect(d, c, e5, false);
}
fn regop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let eg = a.eg.clone().nil();
    if !group::is_contains(&eg, c) {
        return;
    }
    let pe = effect::get_label_object(d, a.e).as_effect().nil();
    effect::set_value(d, pe, 1000);
}
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, a.tp)
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let pe = effect::get_label_object(d, a.e).as_effect().nil();
    let atk = effect::get_value(d, pe).int();
    effect::set_value(d, pe, atk - 200);
}
fn eqop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d);
    if card::is_relate_to_effect(d, c, a.e) {
        if let Some(tc) = tc {
            if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) && card::check_unique_on_field(d, c, a.tp, None, None) {
                duel::equip(d, a.tp, c, tc, true, false);
            }
        }
    }
}
fn eqcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, c, LOCATION_ONFIELD)
        && card::is_previous_position(d, c, POS_FACEUP)
        && card::is_reason(d, c, REASON_DESTROY)
        && card::check_unique_on_field(d, c, a.tp, None, None)
}
fn eqfilter2(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_NOBLE_KNIGHT as u16]) && card::is_race(d, c, RACE_WARRIOR)
}
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && eqfilter2(d, chkc);
    }
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return card::is_relate_to_effect(d, c, a.e)
            && duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&eqfilter2), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    duel::select_target(d, tp, Some(&eqfilter2), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(c)), 1, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
