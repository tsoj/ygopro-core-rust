//! Crystal Beast Topaz Tiger
use super::prelude::*;

pub const ID: u32 = 95600067;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.atkcon)
    effect::set_condition(d, e1, Some(condition(atkcon)));
    // e1:SetValue(400)
    effect::set_value(d, e1, 400);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetCode(EFFECT_TO_GRAVE_REDIRECT_CB)
    effect::set_code(d, e2, EFFECT_TO_GRAVE_REDIRECT_CB);
    // e2:SetCondition(s.replacecon)
    effect::set_condition(d, e2, Some(condition(replacecon)));
    // e2:SetOperation(s.replaceop)
    effect::set_operation(d, e2, Some(operation(replaceop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.atkcon(e)
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetAttacker()==e:GetHandler() and Duel.GetAttackTarget()~=nil and aux.StatChangeDamageStepCondition()
    let c = effect::get_handler(d, a.e).nil();
    duel::get_attacker(d) == Some(c) && duel::get_attack_target(d).is_some() && aux::stat_change_damage_step_condition(d)
}
// function s.replacecon(e)
fn replacecon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsFaceup() and c:IsLocation(LOCATION_MZONE) and c:IsReason(REASON_DESTROY)
    card::is_faceup(d, c) && card::is_location(d, c, LOCATION_MZONE) && card::is_reason(d, c, REASON_DESTROY)
}
// function s.replaceop(e,tp,eg,ep,ev,re,r,rp)
fn replaceop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCode(EFFECT_CHANGE_TYPE)
    effect::set_code(d, e1, EFFECT_CHANGE_TYPE);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetReset((RESET_EVENT|RESETS_STANDARD)&~RESET_TURN_SET)
    effect::set_reset(d, e1, (RESET_EVENT | RESETS_STANDARD) & !RESET_TURN_SET, 1);
    // e1:SetValue(TYPE_SPELL|TYPE_CONTINUOUS)
    effect::set_value(d, e1, (TYPE_SPELL | TYPE_CONTINUOUS) as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Duel.RaiseEvent(c,EVENT_CUSTOM+CARD_CRYSTAL_TREE,e,0,tp,0,0)
    duel::raise_event(d, c, EVENT_CUSTOM + CARD_CRYSTAL_TREE, Some(a.e), 0, a.tp, 0, 0);
}
