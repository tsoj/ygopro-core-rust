//! Imperial Custom
use super::prelude::*;

pub const ID: u32 = 9995766;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Continuous traps cannot be destroyed by battle or card effects
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_target_range(d, e2, LOCATION_ONFIELD, LOCATION_ONFIELD);
    effect::set_target(d, e2, Some(effect_card_fn(infilter)));
    effect::set_value_fn(d, e2, lua_fn(indesval));
    card::register_effect(d, c, e2, false);
}
fn infilter(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_continuous_trap(d, c) && card::get_code(d, c).first().copied() != Some(ID)
}
fn indesval(_d: &mut Duel, a: Args) -> bool {
    let r = a.int(2) as u32;
    (r & (REASON_EFFECT + REASON_BATTLE)) != 0
}
