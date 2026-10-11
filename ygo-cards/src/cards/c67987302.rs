//! Earthbound Linewalker
use super::prelude::*;

pub const ID: u32 = 67987302;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_EARTHBOUND_IMMORTAL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // "Earthbound Immortal" monsters cannot be destroyed by their own effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_EARTHBOUND_IMMORTAL))
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_EARTHBOUND_IMMORTAL as u16]))));
    // e1:SetValue(s.efilter)
    effect::set_value_fn(d, e1, lua_fn(efilter));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.efilter(e,re,rp,c)
fn efilter(d: &mut Duel, a: Args) -> bool {
    // return re:GetOwner()==c
    effect::get_owner(d, a.e(1)) == a.card(3)
}
