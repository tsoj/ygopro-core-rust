//! Royal Command
use super::prelude::*;

pub const ID: u32 = 33950246;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // disable effect
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_CHAIN_ACTIVATING);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_operation(d, e2, Some(operation(disop)));
    card::register_effect(d, c, e2, false);
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // if re:IsActiveType(TYPE_FLIP) then
    let re = a.re.nil();
    if effect::is_active_type(d, re, TYPE_FLIP) {
        // Duel.NegateActivation(ev)
        duel::negate_activation(d, a.ev as u8);
    }
}
