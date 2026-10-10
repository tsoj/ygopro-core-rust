//! Dimensional Fissure
use super::prelude::*;

pub const ID: u32 = 81674782;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // remove
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_SET_AVAILABLE+EFFECT_FLAG_IGNORE_RANGE+EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_SET_AVAILABLE + EFFECT_FLAG_IGNORE_RANGE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetCode(EFFECT_TO_GRAVE_REDIRECT)
    effect::set_code(d, e2, EFFECT_TO_GRAVE_REDIRECT);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTarget(s.rmtarget)
    effect::set_target(d, e2, Some(effect_card_fn(rmtarget)));
    // e2:SetTargetRange(0xff,0xff)
    effect::set_target_range(d, e2, 0xff, 0xff);
    // e2:SetValue(LOCATION_REMOVED)
    effect::set_value(d, e2, LOCATION_REMOVED as i64);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(id)
    effect::set_code(d, e3, ID);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e3, EFFECT_FLAG_SET_AVAILABLE, 0);
    // e3:SetTargetRange(0xff,0xff)
    effect::set_target_range(d, e3, 0xff, 0xff);
    // e3:SetTarget(s.checktg)
    effect::set_target(d, e3, Some(effect_card_fn(checktg)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.rmtarget(e,c)`
fn rmtarget(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsLocation(LOCATION_OVERLAY) and not c:IsSpellTrap()
    !card::is_location(d, c, LOCATION_OVERLAY) && !card::is_spell_trap(d, c)
}
/// `s.checktg(e,c)`
fn checktg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsPublic()
    !card::is_public(d, c)
}
