//! Armored Glass
use super::prelude::*;

pub const ID: u32 = 36868108;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_EQUIP);
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
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
// function s.distarget(e,c)
fn distarget(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsType(TYPE_EQUIP)
    card::is_type(d, c, TYPE_EQUIP)
}
