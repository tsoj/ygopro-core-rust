//! Mesmeric Control
use super::prelude::*;

pub const ID: u32 = 48642904;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_operation(d, e1, Some(operation(regop)));
    card::register_effect(d, c, e1, false);
}
fn regop(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CANNOT_CHANGE_POSITION);
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE | EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 2);
    duel::register_effect(d, e1, a.tp);
}
