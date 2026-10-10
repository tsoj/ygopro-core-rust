//! Non-Spellcasting Area
use super::prelude::*;

pub const ID: u32 = 20065549;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // immune
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_IMMUNE_EFFECT);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_target(d, e2, Some(effect_card_fn(etarget)));
    effect::set_value_fn(d, e2, lua_fn(efilter));
    card::register_effect(d, c, e2, false);
}
/// `s.etarget(e,c)`
fn etarget(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsType(TYPE_EFFECT)
    !card::is_type(d, c, TYPE_EFFECT)
}
/// `s.efilter(e,re)`
fn efilter(d: &mut Duel, a: Args) -> bool {
    // return re:IsSpellEffect()
    effect::is_spell_effect(d, a.e(1))
}
