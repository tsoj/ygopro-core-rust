//! Photon Current
use super::prelude::*;

pub const ID: u32 = 43813459;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e1, EVENT_BE_BATTLE_TARGET);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,...)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local a=Duel.GetAttacker()
    let att = duel::get_attacker(d);
    // local d=Duel.GetAttackTarget()
    let dc = duel::get_attack_target(d);
    // return a:IsOnField() and d and d:IsFaceup() and d:IsControler(tp) and d:IsAttribute(ATTRIBUTE_LIGHT) and d:IsRace(RACE_DRAGON)
    if !card::is_on_field(d, att.nil()) {
        return false;
    }
    match dc {
        Some(dc) => {
            card::is_faceup(d, dc)
                && card::is_controler(d, dc, a.tp)
                && card::is_attribute(d, dc, ATTRIBUTE_LIGHT)
                && card::is_race(d, dc, RACE_DRAGON)
        }
        None => false,
    }
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.GetAttacker():CreateEffectRelation(e)
    card::create_effect_relation(d, duel::get_attacker(d).nil(), a.e);
    // Duel.GetAttackTarget():CreateEffectRelation(e)
    card::create_effect_relation(d, duel::get_attack_target(d).nil(), a.e);
    true
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local a=Duel.GetAttacker()
    let att = duel::get_attacker(d);
    // local d=Duel.GetAttackTarget()
    let dc = duel::get_attack_target(d);
    // if a:IsFacedown() or not a:IsRelateToEffect(e) or d:IsFacedown() or not d:IsRelateToEffect(e) then return end
    if card::is_facedown(d, att.nil())
        || !card::is_relate_to_effect(d, att.nil(), a.e)
        || card::is_facedown(d, dc.nil())
        || !card::is_relate_to_effect(d, dc.nil(), a.e)
    {
        return;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(a:GetAttack())
    let atk = card::get_attack(d, att.nil()) as i64;
    effect::set_value(d, e1, atk);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 1);
    // d:RegisterEffect(e1)
    card::register_effect(d, dc.nil(), e1, false);
}
