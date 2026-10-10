//! Nightmare's Steelcage
use super::prelude::*;

pub const ID: u32 = 58775978;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_POSITION);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    card::register_effect(d, c, e1, false);
    // cannot attack
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_CANNOT_ATTACK_ANNOUNCE);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
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
    // destroy
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_range(d, e1, LOCATION_SZONE);
    effect::set_condition(d, e1, Some(condition(descon)));
    effect::set_operation(d, e1, Some(operation(desop)));
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END | RESET_OPPO_TURN, 2);
    card::register_effect(d, c, e1, false);
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_SET_AVAILABLE, 0);
    effect::set_code(d, e3, 1082946);
    effect::set_label_object(d, e3, Value::Effect(e1));
    effect::set_owner_player(d, e3, a.tp);
    effect::set_operation(d, e3, Some(operation(reset)));
    effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END | RESET_OPPO_TURN, 2);
    card::register_effect(d, c, e3, false);
    true
}
/// `s.reset(e,tp,eg,ep,ev,re,r,rp)`: `s.desop(e:GetLabelObject(),tp,eg,ep,ev,e,r,rp)`
fn reset(d: &mut Duel, a: &EffArgs) {
    let e1 = effect::get_label_object(d, a.e).as_effect().nil();
    desop_body(d, e1, Some(a.e));
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp)
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    desop_body(d, a.e, a.re);
}
fn desop_body(d: &mut Duel, e: EffectId, re: Option<EffectId>) {
    let c = effect::get_handler(d, e).nil();
    let ct = card::get_turn_counter(d, c) + 1;
    card::set_turn_counter(d, c, ct as u16);
    if ct == 2 {
        duel::destroy(d, c, REASON_RULE, None, None);
        if let Some(re) = re {
            effect::reset(d, re);
        }
    }
}
