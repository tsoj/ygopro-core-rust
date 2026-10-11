//! Magical Musket - Steady Hands
use super::prelude::*;

pub const ID: u32 = 67901914;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MAGICAL_MUSKET], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    // e1:SetCondition(aux.StatChangeDamageStepCondition)
    effect::set_condition(d, e1, Some(condition(stat_change_condition)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.GlobalCheck(s,function() ... end)
    aux::global_check(d, ID, |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_ATTACK_ANNOUNCE)
        effect::set_code(d, ge1, EVENT_ATTACK_ANNOUNCE);
        // ge1:SetOperation(s.checkop)
        effect::set_operation(d, ge1, Some(operation(checkop)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
}
// aux.StatChangeDamageStepCondition as the condition
fn stat_change_condition(d: &mut Duel, _a: &EffArgs) -> bool {
    aux::stat_change_damage_step_condition(d)
}
// s.checkop(e,tp,eg,ep,ev,re,r,rp)
fn checkop(d: &mut Duel, a: &EffArgs) {
    // local tc=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let tc = group::get_first(d, &eg).nil();
    // if tc:GetFlagEffect(id)==0 and Duel.GetAttackTarget()==nil then
    if card::get_flag_effect(d, tc, ID) == 0 && duel::get_attack_target(d).is_none() {
        // tc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
        card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    }
}
// s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_MAGICAL_MUSKET) and c:GetFlagEffect(id)==0
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_MAGICAL_MUSKET as u16]) && card::get_flag_effect(d, c, ID) == 0
}
// s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CANNOT_DIRECT_ATTACK)
    effect::set_code(d, e1, EFFECT_CANNOT_DIRECT_ATTACK);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // g:GetFirst():RegisterEffect(e1)
    let tc = group::get_first(d, &g).nil();
    card::register_effect(d, tc, e1, false);
    true
}
// s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(3207)
        effect::set_description(d, e1, 3207);
        // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(tc:GetBaseAttack()*2)
        let atk = card::get_base_attack(d, tc) as i64 * 2;
        effect::set_value(d, e1, atk);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_SET_DEFENSE_FINAL)
        effect::set_code(d, e2, EFFECT_SET_DEFENSE_FINAL);
        // e2:SetValue(tc:GetBaseDefense()*2)
        let def = card::get_base_defense(d, tc) as i64 * 2;
        effect::set_value(d, e2, def);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
    }
}
