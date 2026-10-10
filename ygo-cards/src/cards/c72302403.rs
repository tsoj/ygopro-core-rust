//! Swords of Revealing Light
use super::prelude::*;

pub const ID: u32 = 72302403;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_POSITION);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // cannot attack
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_CANNOT_ATTACK_ANNOUNCE);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    card::register_effect(d, c, e2, false);
    // remain field
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e3, EFFECT_REMAIN_FIELD);
    card::register_effect(d, c, e3, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE);
    }
    let c = effect::get_handler(d, a.e).nil();
    card::set_turn_counter(d, c, 0);
    let sg = duel::get_matching_group(d, Some(&|d, x| card::is_facedown(d, x)), a.tp, 0, LOCATION_MZONE, NO_EX);
    let n = sg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(sg), n, 0, 0);
    // destroy
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_range(d, e1, LOCATION_SZONE);
    effect::set_condition(d, e1, Some(condition(descon)));
    effect::set_operation(d, e1, Some(operation(desop)));
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END | RESET_OPPO_TURN, 3);
    card::register_effect(d, c, e1, false);
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_SET_AVAILABLE, 0);
    effect::set_code(d, e3, 1082946);
    effect::set_label_object(d, e3, Value::Effect(e1));
    effect::set_owner_player(d, e3, a.tp);
    effect::set_operation(d, e3, Some(operation(reset)));
    effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END | RESET_OPPO_TURN, 3);
    card::register_effect(d, c, e3, false);
    true
}
fn reset(d: &mut Duel, a: &EffArgs) {
    let label = effect::get_label_object(d, a.e).as_effect().nil();
    let mut x = a.clone();
    x.e = label;
    x.re = Some(a.e);
    desop(d, &x);
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let sg = duel::get_matching_group(d, Some(&|d, c| card::is_facedown(d, c)), a.tp, 0, LOCATION_MZONE, NO_EX);
    if !sg.is_empty() {
        duel::change_position(d, sg, POS_FACEUP_ATTACK as u8, Some(POS_FACEUP_ATTACK as u8), Some(POS_FACEUP_DEFENSE as u8), Some(POS_FACEUP_DEFENSE as u8), false);
    }
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp)
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let ct = card::get_turn_counter(d, c) + 1;
    card::set_turn_counter(d, c, ct as u16);
    if ct == 3 {
        duel::destroy(d, c, REASON_RULE, None, None);
        if let Some(re) = a.re {
            effect::reset(d, re);
        }
    }
}
