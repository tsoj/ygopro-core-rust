//! V・HERO Trinity
use super::prelude::*;

pub const ID: u32 = 46759931;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_HERO], material_setcode: &[SET_HERO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMixN(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_HERO),3)
    fusion::add_proc_mix_n(
        d,
        c,
        true,
        true,
        vec![(fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_HERO as u16], Some(fc), st, Some(tp)))), 3)],
    );
    // summon success
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(s.regcon)
    effect::set_condition(d, e1, Some(condition(regcon)));
    // e1:SetOperation(s.regop)
    effect::set_operation(d, e1, Some(operation(regop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // extra attack
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_EXTRA_ATTACK)
    effect::set_code(d, e2, EFFECT_EXTRA_ATTACK);
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(condition(atkcon)));
    // e2:SetValue(2)
    effect::set_value(d, e2, 2);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // cannot diratk
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_CANNOT_DIRECT_ATTACK)
    effect::set_code(d, e3, EFFECT_CANNOT_DIRECT_ATTACK);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.regcon(e,tp,eg,ep,ev,re,r,rp)`
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsFusionSummoned()
    let c = effect::get_handler(d, a.e).nil();
    card::is_fusion_summoned(d, c)
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SET_ATTACK)
    effect::set_code(d, e1, EFFECT_SET_ATTACK);
    // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
    // e1:SetValue(c:GetBaseAttack()*2)
    let base = card::get_base_attack(d, c) as i64;
    effect::set_value(d, e1, base * 2);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.atkcon(e)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsFusionSummoned()
    let c = effect::get_handler(d, a.e).nil();
    card::is_fusion_summoned(d, c)
}
