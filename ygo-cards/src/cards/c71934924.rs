//! Swift Samurai Storm!
use super::prelude::*;

pub const ID: u32 = 71934924;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SIX_SAMURAI], ..ScriptInfo::DEFAULT };

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
    // damage cal
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EVENT_BATTLED)
    effect::set_code(d, e2, EVENT_BATTLED);
    // e2:SetOperation(s.atop)
    effect::set_operation(d, e2, Some(operation(atop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // atkup
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e3, EVENT_DAMAGE_STEP_END);
    // e3:SetOperation(s.upop)
    effect::set_operation(d, e3, Some(operation(upop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.check(c,tp)`
fn check(d: &mut Duel, c: Option<CardId>, tp: u8) -> bool {
    // return c and c:IsSetCard(SET_SIX_SAMURAI) and c:IsControler(tp)
    match c {
        Some(c) => card::is_set_card(d, c, &[SET_SIX_SAMURAI as u16]) && card::is_controler(d, c, tp),
        None => false,
    }
}
/// `s.atop(e,tp,eg,ep,ev,re,r,rp)`
fn atop(d: &mut Duel, a: &EffArgs) {
    // if s.check(Duel.GetAttacker(),tp) or s.check(Duel.GetAttackTarget(),tp) then
    let atk = duel::get_attacker(d);
    let hit = check(d, atk, a.tp) || {
        let at = duel::get_attack_target(d);
        check(d, at, a.tp)
    };
    if hit {
        // e:GetHandler():RegisterFlagEffect(id,RESET_PHASE|PHASE_DAMAGE,0,1)
        let h = effect::get_handler(d, a.e).nil();
        card::register_flag_effect(d, h, ID, RESET_PHASE | PHASE_DAMAGE, 0, 1, 0, 0);
    }
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_SIX_SAMURAI) and c:GetFlagEffect(id)==0
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_SIX_SAMURAI as u16]) && card::get_flag_effect(d, c, ID) == 0
}
/// `s.upop(e,tp,eg,ep,ev,re,r,rp)`
fn upop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:GetFlagEffect(id)==0 then return end
    if card::get_flag_effect(d, c, ID) == 0 {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, x| filter(d, x)), a.tp, LOCATION_MZONE, 0, NO_EX);
    // local tc=g:GetFirst()
    let _ = group::get_first(d, &g);
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // --Activate
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(300)
        effect::set_value(d, e1, 300);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // tc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
        card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    }
}
