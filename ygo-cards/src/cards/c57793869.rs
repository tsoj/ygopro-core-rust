//! The Wicked Eraser
use super::prelude::*;

pub const ID: u32 = 57793869;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // summon with 3 tribute
    let _e1 = aux::add_normal_summon_procedure(d, c, true, false, Some(3), Some(3), None, None, None, None);
    let _e2 = aux::add_normal_set_procedure(d, c, true, false, Some(3), Some(3), None, None, None, None);
    // cannot special summon
    let e3 = effect::create_effect(d, c);
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e3, EFFECT_SPSUMMON_CONDITION);
    card::register_effect(d, c, e3, false);
    // atk
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e4, EFFECT_SET_ATTACK);
    effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_value_fn(d, e4, effect_card_value(adval));
    card::register_effect(d, c, e4, false);
    let e5 = effect::clone(d, e4);
    effect::set_code(d, e5, EFFECT_SET_DEFENSE);
    card::register_effect(d, c, e5, false);
    // Eraser
    let e6 = effect::create_effect(d, c);
    effect::set_description(d, e6, aux::stringid(ID, 0));
    effect::set_category(d, e6, CATEGORY_DESTROY);
    effect::set_type(d, e6, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e6, EVENT_TO_GRAVE);
    effect::set_condition(d, e6, Some(condition(erascon)));
    effect::set_target(d, e6, Some(cost_target(erastg)));
    effect::set_operation(d, e6, Some(operation(erasop)));
    card::register_effect(d, c, e6, false);
    // suicide
    let e7 = effect::create_effect(d, c);
    effect::set_description(d, e7, aux::stringid(ID, 1));
    effect::set_type(d, e7, EFFECT_TYPE_IGNITION);
    effect::set_category(d, e7, CATEGORY_DESTROY);
    effect::set_range(d, e7, LOCATION_MZONE);
    effect::set_target(d, e7, Some(cost_target(destg)));
    effect::set_operation(d, e7, Some(operation(desop)));
    card::register_effect(d, c, e7, false);
}
fn adval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return Duel.GetFieldGroupCount(c:GetControler(),0,LOCATION_ONFIELD)*1000
    let p = card::get_controler(d, c);
    duel::get_field_group_count(d, p, 0, LOCATION_ONFIELD) as i64 * 1000
}
fn erascon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsReason(REASON_DESTROY)
    let c = effect::get_handler(d, a.e).nil();
    card::is_reason(d, c, REASON_DESTROY)
}
fn erastg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let tp = a.tp;
    let dg = duel::get_matching_group(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    let n = dg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg), n, 0, 0);
    true
}
fn erasop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let dg = duel::get_matching_group(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    duel::destroy(d, dg, REASON_EFFECT, None, None);
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_destructable(d, c, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    duel::destroy(d, CardSet::from_card(c), REASON_EFFECT, None, None);
}
