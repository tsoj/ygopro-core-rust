//! Ally of Justice Nullfier
use super::prelude::*;

pub const ID: u32 = 76650663;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Disable
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_BATTLED)
    effect::set_code(d, e1, EVENT_BATTLED);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local a=Duel.GetAttacker()
    let mut at = duel::get_attacker(d);
    // if a==c then a=Duel.GetAttackTarget() end
    if at == Some(c) {
        at = duel::get_attack_target(d);
    }
    // e:SetLabelObject(a)
    effect::set_label_object(d, a.e, at.map_or(Value::Nil, Value::Card));
    // return a and a:IsAttribute(ATTRIBUTE_LIGHT) and a:IsRelateToBattle()
    match at {
        Some(at) => card::is_attribute(d, at, ATTRIBUTE_LIGHT) && card::is_relate_to_battle(d, at),
        None => false,
    }
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // if tc:IsFacedown() or not tc:IsRelateToBattle() then return end
    if card::is_facedown(d, tc) || !card::is_relate_to_battle(d, tc) {
        return;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_DISABLE)
    effect::set_code(d, e1, EFFECT_DISABLE);
    // e1:SetReset(RESET_EVENT|(RESETS_STANDARD|RESET_OVERLAY)&~(RESET_LEAVE|RESET_TOGRAVE))
    effect::set_reset(d, e1, RESET_EVENT | ((RESETS_STANDARD | RESET_OVERLAY) & !(RESET_LEAVE | RESET_TOGRAVE)), 1);
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
    // local e2=Effect.CreateEffect(e:GetHandler())
    let e2 = effect::create_effect(d, h);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_DISABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
    // e2:SetReset(RESET_EVENT|(RESETS_STANDARD|RESET_OVERLAY)&~(RESET_LEAVE|RESET_TOGRAVE))
    effect::set_reset(d, e2, RESET_EVENT | ((RESETS_STANDARD | RESET_OVERLAY) & !(RESET_LEAVE | RESET_TOGRAVE)), 1);
    // tc:RegisterEffect(e2)
    card::register_effect(d, tc, e2, false);
}
