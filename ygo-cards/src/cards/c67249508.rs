//! Yang Zing Brutality
use super::prelude::*;

pub const ID: u32 = 67249508;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_YANG_ZING], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e1, EVENT_PRE_DAMAGE_CALCULATE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=Duel.GetAttacker()
    let tc0 = duel::get_attacker(d);
    // local at=Duel.GetAttackTarget()
    let at = duel::get_attack_target(d);
    // if not at or tc:IsFacedown() or at:IsFacedown() then return false end
    let Some(at) = at else {
        return false;
    };
    let mut tc = tc0.nil();
    if card::is_facedown(d, tc) || card::is_facedown(d, at) {
        return false;
    }
    // if tc:IsControler(1-tp) then tc=at end
    if card::is_controler(d, tc, 1 - a.tp) {
        tc = at;
    }
    // e:SetLabelObject(tc)
    effect::set_label_object(d, a.e, Value::Card(tc));
    // return tc:IsControler(tp) and tc:IsLocation(LOCATION_MZONE) and tc:IsSetCard(SET_YANG_ZING)
    card::is_controler(d, tc, a.tp) && card::is_location(d, tc, LOCATION_MZONE) && card::is_set_card(d, tc, &[SET_YANG_ZING as u16])
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // if tc:IsRelateToBattle() and not tc:IsImmuneToEffect(e) then
    if card::is_relate_to_battle(d, tc) && !card::is_immune_to_effect(d, tc, a.e) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(tc:GetBaseAttack()*2)
        let atk = card::get_base_attack(d, tc);
        effect::set_value(d, e1, (atk * 2) as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE_CAL)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE_CAL, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_SET_DEFENSE_FINAL)
        effect::set_code(d, e2, EFFECT_SET_DEFENSE_FINAL);
        // e2:SetValue(tc:GetBaseDefense()*2)
        let def = card::get_base_defense(d, tc);
        effect::set_value(d, e2, (def * 2) as i64);
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE_CAL)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE_CAL, 1);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
        // local fid=c:GetFieldID()
        let fid = card::get_field_id(d, c);
        // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1,fid)
        card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, fid as i64, 0);
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e3:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
        effect::set_property(d, e3, EFFECT_FLAG_IGNORE_IMMUNE, 0);
        // e3:SetCode(EVENT_DAMAGE_STEP_END)
        effect::set_code(d, e3, EVENT_DAMAGE_STEP_END);
        // e3:SetCountLimit(1)
        effect::set_count_limit(d, e3, 1, 0, 0, 0);
        // e3:SetLabel(fid)
        effect::set_label(d, e3, &[fid as i64]);
        // e3:SetLabelObject(tc)
        effect::set_label_object(d, e3, Value::Card(tc));
        // e3:SetCondition(s.descon)
        effect::set_condition(d, e3, Some(condition(descon)));
        // e3:SetOperation(s.desop)
        effect::set_operation(d, e3, Some(operation(desop)));
        // e3:SetReset(RESET_PHASE|PHASE_DAMAGE)
        effect::set_reset(d, e3, RESET_PHASE | PHASE_DAMAGE, 1);
        // Duel.RegisterEffect(e3,tp)
        duel::register_effect(d, e3, a.tp);
    }
}
// function s.descon(e,tp,eg,ep,ev,re,r,rp)
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // if tc:GetFlagEffectLabel(id)==e:GetLabel() then
    if card::get_flag_effect_label(d, tc, ID).first().copied() == Some(effect::get_label(d, a.e)) {
        // return true
        true
    } else {
        // e:Reset()
        effect::reset(d, a.e);
        // return false
        false
    }
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // Duel.Destroy(tc,REASON_EFFECT)
    duel::destroy(d, tc, REASON_EFFECT, None, None);
}
