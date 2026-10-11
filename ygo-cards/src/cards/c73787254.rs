//! Saber Vault
use super::prelude::*;

pub const ID: u32 = 73787254;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_X_SABER], ..ScriptInfo::DEFAULT };

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
    // atk,def
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_X_SABER))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_X_SABER as u16]))));
    // e2:SetValue(s.val1)
    effect::set_value_fn(d, e2, effect_card_value(val1));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    // e3:SetValue(s.val2)
    effect::set_value_fn(d, e3, effect_card_value(val2));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.val1(e,c)`
fn val1(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return c:GetLevel()*100
    card::get_level(d, c) as i64 * 100
}
/// `s.val2(e,c)`
fn val2(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return -c:GetLevel()*100
    -(card::get_level(d, c) as i64) * 100
}
