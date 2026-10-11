//! Ally of Justice Quarantine
use super::prelude::*;

pub const ID: u32 = 72845813;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // disable spsummon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(1,1)
    effect::set_target_range(d, e1, 1, 1);
    // e1:SetTarget(aux.TargetBoolFunction(Card.IsAttribute,ATTRIBUTE_LIGHT))
    effect::set_target(d, e1, Some(effect_card_fn(target)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// aux.TargetBoolFunction(Card.IsAttribute,ATTRIBUTE_LIGHT): function(e,c) return Card.IsAttribute(c,ATTRIBUTE_LIGHT) end
fn target(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_attribute(d, c, ATTRIBUTE_LIGHT)
}
