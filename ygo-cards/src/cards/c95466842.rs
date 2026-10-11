//! Mermail Abysslung
//! 水精鱗－アビスラング
use super::prelude::*;

pub const ID: u32 = 95466842;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // at limit
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    // e1:SetValue(s.atlimit)
    effect::set_value_fn(d, e1, effect_card_fn(atlimit));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // atkup
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsAttribute,ATTRIBUTE_WATER))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_attribute(d, c, ATTRIBUTE_WATER))));
    // e2:SetValue(300)
    effect::set_value(d, e2, 300);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.atlimit(e,c)
fn atlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c~=e:GetHandler() and c:IsFaceup() and c:IsAttribute(ATTRIBUTE_WATER)
    let h = effect::get_handler(d, e).nil();
    c != h && card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_WATER)
}
