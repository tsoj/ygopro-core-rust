//! Wattgiraffe
use super::prelude::*;

pub const ID: u32 = 402568;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Can attack directly
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
    card::register_effect(d, c, e1, false);
    // Make your opponent unable to activate cards or effects that turn
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_BATTLE_DAMAGE);
    effect::set_condition(d, e2, Some(condition(cond)));
    effect::set_operation(d, e2, Some(operation(op)));
    card::register_effect(d, c, e2, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    a.ep == 1 - a.tp && duel::get_attack_target(d).is_none()
}
fn op(d: &mut Duel, a: &EffArgs) {
    // Cannot activate cards or effects until the end of this turn
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 1));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    effect::set_target_range(d, e1, 0, 1);
    effect::set_value(d, e1, 1);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
}
