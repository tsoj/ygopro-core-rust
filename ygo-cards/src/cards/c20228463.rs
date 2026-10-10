//! Ceremonial Bell
use super::prelude::*;

pub const ID: u32 = 20228463;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_PUBLIC)
    effect::set_code(d, e1, EFFECT_PUBLIC);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_HAND,LOCATION_HAND)
    effect::set_target_range(d, e1, LOCATION_HAND, LOCATION_HAND);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
