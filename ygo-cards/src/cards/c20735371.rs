//! Bye Bye Damage
use super::prelude::*;

pub const ID: u32 = 20735371;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e1, EVENT_PRE_DAMAGE_CALCULATE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,...)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local t=Duel.GetAttackTarget()
    // return t and t:IsControler(tp) and Duel.GetFlagEffect(tp,id)==0
    match duel::get_attack_target(d) {
        Some(t) => card::is_controler(d, t, a.tp) && duel::get_flag_effect(d, a.tp, ID) == 0,
        None => false,
    }
}
/// `s.activate(e,tp,...)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e);
    // local tc=Duel.GetAttackTarget()
    let tc = duel::get_attack_target(d);
    // if tc and tc:IsRelateToBattle() and tc:IsFaceup() then
    if let Some(tc) = tc {
        if card::is_relate_to_battle(d, tc) && card::is_faceup(d, tc) {
            // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
            duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c.nil());
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
            effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
            // e1:SetValue(1)
            effect::set_value(d, e1, 1);
            // e1:SetReset(RESET_PHASE|PHASE_DAMAGE)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c.nil());
            // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            // e2:SetCode(EVENT_BATTLE_DAMAGE)
            effect::set_code(d, e2, EVENT_BATTLE_DAMAGE);
            // e2:SetRange(LOCATION_MZONE)
            effect::set_range(d, e2, LOCATION_MZONE);
            // e2:SetCondition(s.damcon)
            effect::set_condition(d, e2, Some(condition(damcon)));
            // e2:SetOperation(s.damop)
            effect::set_operation(d, e2, Some(operation(damop)));
            // e2:SetReset(RESET_PHASE|PHASE_DAMAGE)
            effect::set_reset(d, e2, RESET_PHASE | PHASE_DAMAGE, 1);
            // Duel.RegisterEffect(e2,tp)
            duel::register_effect(d, e2, a.tp);
        }
    }
}
/// `s.damcon(e,tp,eg,ep,...)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==tp and eg:GetFirst():IsRelateToBattle()
    if a.ep != a.tp {
        return false;
    }
    let g = a.eg.clone().nil();
    let tc = group::get_first(d, &g).nil();
    card::is_relate_to_battle(d, tc)
}
/// `s.damop(e,tp,eg,ep,ev,...)`
fn damop(d: &mut Duel, a: &EffArgs) {
    // Duel.Damage(1-tp,ev*2,REASON_EFFECT)
    duel::damage(d, 1 - a.tp, (a.ev * 2) as i64, REASON_EFFECT, false, None);
}
