//! Reverse Trap
use super::prelude::*;

pub const ID: u32 = 77622396;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_REVERSE_UPDATE);
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    duel::register_effect(d, e1, a.tp);
}
