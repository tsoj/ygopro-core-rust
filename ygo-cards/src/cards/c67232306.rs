//! Mirror Mail
use super::prelude::*;

pub const ID: u32 = 67232306;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atk
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e1, EVENT_BE_BATTLE_TARGET);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local at=eg:GetFirst()
    let at = group::get_first(d, &a.eg.clone().nil());
    // local a=Duel.GetAttacker()
    let att = duel::get_attacker(d);
    // if chk==0 then return at:IsControler(tp) and at:IsOnField() and at:IsFaceup() and a:IsOnField() end
    if a.chk == Some(0) {
        let at = at.nil();
        return card::is_controler(d, at, a.tp) && card::is_on_field(d, at) && card::is_faceup(d, at) && card::is_on_field(d, att.nil());
    }
    // at:CreateEffectRelation(e)
    card::create_effect_relation(d, at.nil(), a.e);
    // a:CreateEffectRelation(e)
    card::create_effect_relation(d, att.nil(), a.e);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local at=eg:GetFirst()
    let at = group::get_first(d, &a.eg.clone().nil());
    // local a=Duel.GetAttacker()
    let att = duel::get_attacker(d);
    // if not a:IsRelateToEffect(e) or not at:IsRelateToEffect(e) or a:IsFacedown() or at:IsFacedown() then return end
    if !card::is_relate_to_effect(d, att.nil(), a.e)
        || !card::is_relate_to_effect(d, at.nil(), a.e)
        || card::is_facedown(d, att.nil())
        || card::is_facedown(d, at.nil())
    {
        return;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let handler = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, handler);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
    effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
    // e1:SetValue(a:GetAttack())
    let atk = card::get_attack(d, att.nil());
    effect::set_value(d, e1, atk as i64);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // at:RegisterEffect(e1)
    card::register_effect(d, at.nil(), e1, false);
}
