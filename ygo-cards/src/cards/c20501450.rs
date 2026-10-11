//! Concentrating Current
use super::prelude::*;

pub const ID: u32 = 20501450;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atk gain
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(aux.StatChangeDamageStepCondition)
    effect::set_condition(d, e1, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, |d| {
        // s[0]=0
        set_s_var(d, ID, 0, Value::Int(0));
        // s[1]=0
        set_s_var(d, ID, 1, Value::Int(0));
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
        // local ge2=Effect.CreateEffect(c)
        let ge2 = effect::create_effect(d, c);
        // ge2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge2:SetCode(EVENT_ADJUST)
        effect::set_code(d, ge2, EVENT_ADJUST);
        // ge2:SetCountLimit(1)
        effect::set_count_limit(d, ge2, 1, 0, 0, 0);
        // ge2:SetOperation(s.clear)
        effect::set_operation(d, ge2, Some(operation(clear)));
        // Duel.RegisterEffect(ge2,0)
        duel::register_effect(d, ge2, 0);
    });
}
/// `s.checkop(e,tp,eg,ep,ev,re,r,rp)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    // local tc=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let tc = group::get_first(d, &eg).nil();
    // if tc:GetFlagEffect(id)==0 then
    if card::get_flag_effect(d, tc, ID) == 0 {
        // s[ep]=s[ep]+1
        let ep = a.ep as u64;
        let v = s_var(d, ID, ep).int();
        set_s_var(d, ID, ep, Value::Int(v + 1));
        // tc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
        card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    }
}
/// `s.clear(e,tp,eg,ep,ev,re,r,rp)`
fn clear(d: &mut Duel, _a: &EffArgs) {
    // s[0]=0
    set_s_var(d, ID, 0, Value::Int(0));
    // s[1]=0
    set_s_var(d, ID, 1, Value::Int(0));
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // return true
    true
}
/// `s.filter(c,tp,costchk)`
fn filter(d: &mut Duel, c: CardId, tp: u8, costchk: bool) -> bool {
    // return c:IsFaceup() and c:HasNonZeroDefense() and (not costchk or s[tp]==0 or c:GetFlagEffect(id)~=0)
    card::is_faceup(d, c)
        && card::has_non_zero_defense(d, c)
        && (!costchk || s_var(d, ID, tp as u64).int() == 0 || card::get_flag_effect(d, c, ID) != 0)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local costchk=e:GetLabel()==1
    let costchk = effect::get_label(d, a.e) == 1;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.filter(chkc,tp,costchk) and chkc:IsControler(tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc, a.tp, costchk) && card::is_controler(d, chkc, a.tp);
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // if costchk and s[tp]>=2 then return false end
        if costchk && s_var(d, ID, a.tp as u64).int() >= 2 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil,tp,costchk)
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, a.tp, costchk)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil,tp,costchk)
    let g = duel::select_target(d, a.tp, Some(&|d, c| filter(d, c, a.tp, costchk)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // if costchk then
    if costchk {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetCode(EFFECT_CANNOT_ATTACK)
        effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
        // e1:SetProperty(EFFECT_FLAG_OATH)
        effect::set_property(d, e1, EFFECT_FLAG_OATH, 0);
        // e1:SetTargetRange(LOCATION_MZONE,0)
        effect::set_target_range(d, e1, LOCATION_MZONE, 0);
        // e1:SetTarget(s.ftarget)
        effect::set_target(d, e1, Some(effect_card_fn(ftarget)));
        // e1:SetLabel(g:GetFirst():GetFieldID())
        let first = group::get_first(d, &g).nil();
        let fid = card::get_field_id(d, first) as i64;
        effect::set_label(d, e1, &[fid]);
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
        // aux.RegisterClientHint(e:GetHandler(),nil,tp,0,1,aux.Stringid(id,1),nil)
        let h = effect::get_handler(d, a.e).nil();
        aux::register_client_hint(d, h, 0, a.tp, 0, 1, Some(aux::stringid(ID, 1)), 0, None);
    }
    true
}
/// `s.ftarget(e,c)`
fn ftarget(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return e:GetLabel()~=c:GetFieldID()
    effect::get_label(d, e) != card::get_field_id(d, c) as i64
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // e1:SetValue(tc:GetDefense())
        let def = card::get_defense(d, tc);
        effect::set_value(d, e1, def as i64);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
