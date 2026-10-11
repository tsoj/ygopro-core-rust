//! Condemned Maiden
use super::prelude::*;

pub const ID: u32 = 68018709;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // act qp in hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_QP_ACT_IN_NTPHAND)
    effect::set_code(d, e1, EFFECT_QP_ACT_IN_NTPHAND);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_HAND,0)
    effect::set_target_range(d, e1, LOCATION_HAND, 0);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_DUEL)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_DUEL as u8);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
