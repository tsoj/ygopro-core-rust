//! General Grunard of the Ice Barrier
use super::prelude::*;

pub const ID: u32 = 9056100;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ICE_BARRIER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_HAND|LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_HAND | LOCATION_MZONE, 0);
    // e1:SetCode(EFFECT_EXTRA_SUMMON_COUNT)
    effect::set_code(d, e1, EFFECT_EXTRA_SUMMON_COUNT);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_ICE_BARRIER))
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_ICE_BARRIER as u16]))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
