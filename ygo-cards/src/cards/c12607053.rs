//! Waboku
use super::prelude::*;

pub const ID: u32 = 12607053;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let h1 = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h1);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_AVOID_BATTLE_DAMAGE);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_value(d, e1, 1);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
    let h2 = effect::get_handler(d, a.e).nil();
    let e2 = effect::create_effect(d, h2);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    effect::set_value(d, e2, 1);
    duel::register_effect(d, e2, a.tp);
}
