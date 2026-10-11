//! Cipher Interference
use super::prelude::*;

pub const ID: u32 = 55496220;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CIPHER], ..ScriptInfo::DEFAULT };

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
    // Double ATK
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e2, EVENT_PRE_DAMAGE_CALCULATE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(condition(atkcon)));
    // e2:SetTarget(s.atktg)
    effect::set_target(d, e2, Some(cost_target(atktg)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.filter(c,code)
fn filter(d: &mut Duel, c: CardId, code: u32) -> bool {
    // return c:IsFaceup() and c:IsCode(code)
    card::is_faceup(d, c) && card::is_code(d, c, &[code])
}
// function s.atkcon(e,tp,eg,ep,ev,re,r,rp)
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=Duel.GetAttacker()
    let attacker = duel::get_attacker(d);
    // if not tc then return false end
    let Some(attacker) = attacker else { return false };
    let mut tc = Some(attacker);
    // if tc:IsControler(1-tp) then
    if card::is_controler(d, attacker, 1 - a.tp) {
        // tc=Duel.GetAttackTarget()
        tc = duel::get_attack_target(d);
    }
    // e:SetLabelObject(tc)
    effect::set_label_object(d, a.e, tc.map_or(Value::Nil, Value::Card));
    // return tc and tc:IsFaceup() and tc:IsSetCard(SET_CIPHER) and tc:IsRelateToBattle()
    //     and Duel.IsExistingMatchingCard(s.filter,0,LOCATION_MZONE,LOCATION_MZONE,1,tc,tc:GetCode())
    let Some(tc) = tc else { return false };
    if !card::is_faceup(d, tc) || !card::is_set_card(d, tc, &[SET_CIPHER as u16]) || !card::is_relate_to_battle(d, tc) {
        return false;
    }
    let code = card::get_code(d, tc)[0];
    duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, code)), 0, LOCATION_MZONE, LOCATION_MZONE, 1, tc.into())
}
// function s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card();
    // if chk==0 then return tc and tc:IsOnField() end
    if a.chk == Some(0) {
        return match tc {
            Some(tc) => card::is_on_field(d, tc),
            None => false,
        };
    }
    // e:SetLabelObject(nil)
    effect::set_label_object(d, a.e, Value::Nil);
    // Duel.SetTargetCard(tc)
    duel::set_target_card(d, CardSet::from_card(tc.nil()));
    true
}
// function s.atkop(e,tp,eg,ep,ev,re,r,rp,chk)
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if not c:IsRelateToEffect(e) or not tc or not tc:IsRelateToBattle() or tc:IsFacedown() then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let Some(tc) = tc else { return };
    if !card::is_relate_to_battle(d, tc) || card::is_facedown(d, tc) {
        return;
    }
    // Double ATK
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
    effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_BATTLE)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_BATTLE, 1);
    // e1:SetValue(tc:GetAttack()*2)
    let atk = card::get_attack(d, tc) as i64;
    effect::set_value(d, e1, atk * 2);
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
}
