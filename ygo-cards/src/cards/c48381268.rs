//! Morphtronic Boarden
use super::prelude::*;

pub const ID: u32 = 48381268;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MORPHTRONIC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atk
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
    // e1:SetCondition(s.cona)
    effect::set_condition(d, e1, Some(condition(cona)));
    // e1:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_MORPHTRONIC))
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_MORPHTRONIC as u16]))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // def
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    // e2:SetCondition(s.cond)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetTarget(s.tgd)
    effect::set_target(d, e2, Some(effect_card_fn(tgd)));
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cona(e)`
fn cona(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsAttackPos()
    let h = effect::get_handler(d, a.e).nil();
    card::is_attack_pos(d, h)
}
/// `s.cond(e)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsDefensePos()
    let h = effect::get_handler(d, a.e).nil();
    card::is_defense_pos(d, h)
}
/// `s.tgd(e,c)`
fn tgd(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:IsSetCard(SET_MORPHTRONIC) and c~=e:GetHandler()
    card::is_set_card(d, c, &[SET_MORPHTRONIC as u16]) && c != effect::get_handler(d, e).nil()
}
