//! Metal Detector
use super::prelude::*;

pub const ID: u32 = 75646520;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:GetActiveType()==TYPE_CONTINUOUS+TYPE_TRAP
    let re = a.re.nil();
    effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && effect::get_active_type(d, re) == TYPE_CONTINUOUS + TYPE_TRAP
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    // disable
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_DISABLE);
    effect::set_target_range(d, e1, LOCATION_SZONE, LOCATION_SZONE);
    effect::set_target(d, e1, Some(effect_card_fn(distarget)));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
}
fn distarget(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsContinuousTrap()
    card::is_continuous_trap(d, c)
}
