//! Samsara
use super::prelude::*;

pub const ID: u32 = 44182827;

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
    // e2:SetValue(LOCATION_DECKSHF)
    effect::set_value(d, e2, LOCATION_DECKSHF as i64);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.rmtarget(e,c)`
fn rmtarget(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:GetReason()==REASON_RELEASE+REASON_RITUAL+REASON_EFFECT+REASON_MATERIAL
    card::get_reason(d, c) == REASON_RELEASE + REASON_RITUAL + REASON_EFFECT + REASON_MATERIAL
}
