//! Energy Bravery
use super::prelude::*;

pub const ID: u32 = 72631243;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Gemini monsters cannot be destroyed by effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(function(_,c) return c:IsGeminiStatus() end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, cc| card::is_gemini_state(d, cc))));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
