//! Prepare to Strike Back
use super::prelude::*;

pub const ID: u32 = 4483989;

pub const INFO: ScriptInfo = ScriptInfo { toss_coin: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_BATTLE_START)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_BATTLE_START as u32));
    // e1:SetTarget(s.atktg1)
    effect::set_target(d, e1, Some(cost_target(atktg1)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Toss a coin and apply the appropriate effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_COIN)
    effect::set_category(d, e2, CATEGORY_COIN);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(condition(atkcon)));
    // e2:SetTarget(s.atktg2)
    effect::set_target(d, e2, Some(cost_target(atktg2)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local at=Duel.GetAttackTarget()
    let at = duel::get_attack_target(d);
    // return Duel.IsTurnPlayer(1-tp) and at and at:IsPosition(POS_FACEUP_DEFENSE)
    duel::is_turn_player(d, 1 - a.tp) && at.is_some_and(|at| card::is_position(d, at, POS_FACEUP_DEFENSE))
}
/// `s.atktg1(e,tp,...,chk)`
fn atktg1(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // e:SetLabel(0)
    effect::set_label(d, a.e, &[0]);
    // local at=Duel.GetAttackTarget()
    let at = duel::get_attack_target(d);
    // if Duel.CheckEvent(EVENT_ATTACK_ANNOUNCE) and Duel.IsTurnPlayer(1-tp)
    //    and at and at:IsPosition(POS_FACEUP_DEFENSE) then
    if duel::check_event(d, EVENT_ATTACK_ANNOUNCE).is_some()
        && duel::is_turn_player(d, 1 - a.tp)
        && at.is_some_and(|at| card::is_position(d, at, POS_FACEUP_DEFENSE))
    {
        let at = at.unwrap();
        // e:SetLabel(1)
        effect::set_label(d, a.e, &[1]);
        // Duel.GetAttacker():CreateEffectRelation(e)
        let attacker = duel::get_attacker(d).nil();
        card::create_effect_relation(d, attacker, a.e);
        // at:CreateEffectRelation(e)
        card::create_effect_relation(d, at, a.e);
        // Duel.SetOperationInfo(0,CATEGORY_COIN,nil,0,tp,1)
        duel::set_operation_info(d, 0, CATEGORY_COIN, None, 0, a.tp, 1);
    }
    true
}
/// `s.atktg2(e,tp,...,chk)`
fn atktg2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // Duel.GetAttacker():CreateEffectRelation(e)
    let attacker = duel::get_attacker(d).nil();
    card::create_effect_relation(d, attacker, a.e);
    // Duel.GetAttackTarget():CreateEffectRelation(e)
    let target = duel::get_attack_target(d).nil();
    card::create_effect_relation(d, target, a.e);
    // Duel.SetOperationInfo(0,CATEGORY_COIN,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_COIN, None, 0, a.tp, 1);
    true
}
/// `s.atkop(e,tp,...)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // if e:GetLabel()==0 or not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if effect::get_label(d, a.e) == 0 || !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local a=Duel.GetAttacker()
    let att = duel::get_attacker(d).nil();
    // local at=Duel.GetAttackTarget()
    let at = duel::get_attack_target(d);
    // if a:IsFaceup() and a:IsRelateToEffect(e) and at:IsFaceup() and at:IsRelateToEffect(e) then
    if card::is_faceup(d, att)
        && card::is_relate_to_effect(d, att, a.e)
        && card::is_faceup(d, at.nil())
        && card::is_relate_to_effect(d, at.nil(), a.e)
    {
        let at = at.nil();
        // if Duel.CallCoin(tp) then
        if duel::call_coin(d, a.tp) {
            // Duel.ChangePosition(at,POS_FACEUP_ATTACK)
            duel::change_position(d, at, POS_FACEUP_ATTACK as u8, None, None, None, false);
        // elseif a:GetAttack()>at:GetDefense() then
        } else if card::get_attack(d, att) > card::get_defense(d, at) {
            // Duel.Damage(tp,a:GetAttack()-at:GetDefense(),REASON_EFFECT)
            let amount = card::get_attack(d, att) - card::get_defense(d, at);
            duel::damage(d, a.tp, amount as i64, REASON_EFFECT, false, None);
        }
    }
}
