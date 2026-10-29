//! A・ジェネクス・ベルフレイム
//! Genex Ally Bellflame
use super::prelude::*;

pub const ID: u32 = 91438994;

pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[0xa], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_counter_permit(d, c, 0xa, None, None);
    // add counter
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_operation(d, e1, Some(operation(addc1)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_REMOVE);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_operation(d, e2, Some(operation(addc2)));
    card::register_effect(d, c, e2, false);
    // attackup
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    effect::set_value_fn(d, e3, effect_value(attackup));
    card::register_effect(d, c, e3, false);
    // damage
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e4, EVENT_LEAVE_FIELD_P);
    effect::set_operation(d, e4, Some(operation(damp)));
    card::register_effect(d, c, e4, false);
    let e5 = effect::create_effect(d, c);
    effect::set_description(d, e5, aux::stringid(ID, 0));
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_category(d, e5, CATEGORY_DAMAGE);
    effect::set_property(d, e5, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e5, EVENT_BATTLE_DESTROYED);
    effect::set_condition(d, e5, Some(condition(damcon)));
    effect::set_target(d, e5, Some(cost_target(damtg)));
    effect::set_operation(d, e5, Some(operation(damop)));
    effect::set_label_object(d, e5, Value::Effect(e4));
    card::register_effect(d, c, e5, false);
}
fn filter1(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_previous_controler(d, c, tp) && card::get_previous_location(d, c) == LOCATION_MZONE
}
fn addc1(d: &mut Duel, a: &EffArgs) {
    let eg = a.eg.clone().nil();
    if group::is_exists(d, &eg, &|d, c| filter1(d, c, a.tp), 1, NO_EX) {
        let h = effect::get_handler(d, a.e).nil();
        card::add_counter(d, h, 0xa, 1, false);
    }
}
fn filter2(d: &mut Duel, c: CardId, tp: u8) -> bool {
    !card::is_previous_controler(d, c, tp) && card::get_previous_location(d, c) == LOCATION_GRAVE
}
fn addc2(d: &mut Duel, a: &EffArgs) {
    let eg = a.eg.clone().nil();
    if group::is_exists(d, &eg, &|d, c| filter2(d, c, a.tp), 1, NO_EX) {
        let h = effect::get_handler(d, a.e).nil();
        card::add_counter(d, h, 0xa, 2, false);
    }
}
fn attackup(d: &mut Duel, _e: EffectId) -> i64 {
    duel::get_counter(d, 0, 1, 1, 0xa) as i64 * 100
}
fn damp(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    let v = card::get_counter(d, h, 0xa);
    effect::set_label(d, a.e, &[v as i64]);
}
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_location(d, h, LOCATION_GRAVE) && card::is_reason(d, h, REASON_BATTLE)
}
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    let ct = effect::get_label(d, lo);
    if a.chk == Some(0) {
        return ct != 0;
    }
    duel::set_target_player(d, 1 - a.tp);
    duel::set_target_param(d, (ct * 300) as u32);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, (ct * 300) as i32);
    true
}
fn damop(d: &mut Duel, _a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
