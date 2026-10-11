//! Roar of the Earthbound Immortal
use super::prelude::*;

pub const ID: u32 = 56339050;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_EARTHBOUND_IMMORTAL], ..ScriptInfo::DEFAULT };

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
    // Destroy the attacking monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e2, CATEGORY_DESTROY | CATEGORY_DAMAGE);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD | EFFECT_TYPE_TRIGGER_F);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.cfilter(c,atk)
fn cfilter(d: &mut Duel, c: CardId, atk: i32) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_EARTHBOUND_IMMORTAL) and c:GetAttack()>atk
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_EARTHBOUND_IMMORTAL as u16]) && card::get_attack(d, c) > atk
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=Duel.GetAttacker()
    let tc = duel::get_attacker(d);
    // return Duel.IsTurnPlayer(1-tp) and Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil,tc:GetAttack())
    if !duel::is_turn_player(d, 1 - a.tp) {
        return false;
    }
    let atk = card::get_attack(d, tc.nil());
    duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c, atk)), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=Duel.GetAttacker()
    let tc = duel::get_attacker(d);
    // if chkc then return chkc==tc end
    if let Some(chkc) = a.chkc() {
        return Some(chkc) == tc;
    }
    // if chk==0 then return tc:IsOnField() and tc:IsCanBeEffectTarget(e) end
    let tc = tc.nil();
    if a.chk == Some(0) {
        return card::is_on_field(d, tc) && card::is_can_be_effect_target(d, tc, Some(a.e));
    }
    // Duel.SetTargetCard(tc)
    duel::set_target_card(d, tc);
    // local dam=tc:GetAttack()/2
    let dam = card::get_attack(d, tc) as f64 / 2.0;
    // Duel.SetTargetParam(dam)
    duel::set_target_param(d, lua_int(dam) as u32);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(tc)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,dam)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, lua_int(dam) as i32);
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if not Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil,tc:GetAttack()) then return end
    let atk = card::get_attack(d, tc.nil());
    if !duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c, atk)), a.tp, LOCATION_MZONE, 0, 1, NO_EX) {
        return;
    }
    let tc = tc.nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() and tc:CanAttack() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) && card::can_attack(d, tc) {
        // local atk=tc:GetAttack()/2
        let atk = card::get_attack(d, tc) as f64 / 2.0;
        // if Duel.Destroy(tc,REASON_EFFECT)>0 and atk>0 then
        if duel::destroy(d, tc, REASON_EFFECT, None, None) > 0 && atk > 0.0 {
            // Duel.Damage(1-tp,atk,REASON_EFFECT)
            duel::damage(d, 1 - a.tp, lua_int(atk), REASON_EFFECT, false, None);
        }
    }
}
