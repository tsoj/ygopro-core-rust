//! Witch's Apprentice
use super::prelude::*;

pub const ID: u32 = 80741828;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetTarget(s.tg1)
    effect::set_target(d, e1, Some(effect_card_fn(tg1)));
    // e1:SetValue(500)
    effect::set_value(d, e1, 500);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetTarget(s.tg2)
    effect::set_target(d, e2, Some(effect_card_fn(tg2)));
    // e2:SetValue(-400)
    effect::set_value(d, e2, -400);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.tg1(e,c)
fn tg1(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_DARK)
    card::is_attribute(d, c, ATTRIBUTE_DARK)
}
// function s.tg2(e,c)
fn tg2(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT)
    card::is_attribute(d, c, ATTRIBUTE_LIGHT)
}
