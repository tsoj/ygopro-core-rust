//! Final Fusion
use super::prelude::*;

pub const ID: u32 = 89719143;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_BATTLE_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_BATTLE_PHASE));
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    // local a=Duel.GetAttacker()
    let a = duel::get_attacker(d);
    // local at=Duel.GetAttackTarget()
    let at = duel::get_attack_target(d);
    // return a and at and a:IsFaceup() and a:IsType(TYPE_FUSION) and at:IsFaceup() and at:IsType(TYPE_FUSION)
    let (Some(a), Some(at)) = (a, at) else { return false };
    card::is_faceup(d, a) && card::is_type(d, a, TYPE_FUSION) && card::is_faceup(d, at) && card::is_type(d, at, TYPE_FUSION)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local a=Duel.GetAttacker()
    let atk = duel::get_attacker(d);
    // local at=Duel.GetAttackTarget()
    let at = duel::get_attack_target(d);
    // if chkc then return chkc==a or chkc==at end
    if let Some(chkc) = a.chkc() {
        return Some(chkc) == atk || Some(chkc) == at;
    }
    // if chk==0 then return a:IsOnField() and a:IsCanBeEffectTarget(e) and at:IsOnField() and at:IsCanBeEffectTarget(e) end
    let a1 = atk.nil();
    if a.chk == Some(0) {
        let at1 = at.nil();
        return card::is_on_field(d, a1)
            && card::is_can_be_effect_target(d, a1, Some(a.e))
            && card::is_on_field(d, at1)
            && card::is_can_be_effect_target(d, at1, Some(a.e));
    }
    // Duel.SetTargetCard(a)
    duel::set_target_card(d, a1);
    // Duel.SetTargetCard(at)
    let at1 = at.nil();
    duel::set_target_card(d, at1);
    // local dam=a:GetAttack()+at:GetAttack()
    let dam = card::get_attack(d, a1) + card::get_attack(d, at1);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,PLAYER_ALL,dam)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, PLAYER_ALL as u8, dam);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc1,tc2=Duel.GetFirstTarget()
    let targets = duel::get_first_target(d);
    let tc1 = targets.first().copied().nil();
    let tc2 = targets.get(1).copied().nil();
    // local dam=tc1:GetAttack()+tc2:GetAttack()
    let dam = card::get_attack(d, tc1) + card::get_attack(d, tc2);
    // if Duel.NegateAttack() then
    if duel::negate_attack(d) {
        // if tc1:IsRelateToEffect(e) and tc1:IsFaceup() and tc2:IsRelateToEffect(e) and tc2:IsFaceup() then
        if card::is_relate_to_effect(d, tc1, a.e) && card::is_faceup(d, tc1) && card::is_relate_to_effect(d, tc2, a.e) && card::is_faceup(d, tc2) {
            // Duel.Damage(1-tp,dam,REASON_EFFECT,true)
            duel::damage(d, 1 - tp, dam as i64, REASON_EFFECT, true, None);
            // Duel.Damage(tp,dam,REASON_EFFECT,true)
            duel::damage(d, tp, dam as i64, REASON_EFFECT, true, None);
            // Duel.RDComplete()
            duel::rd_complete(d);
        }
    }
}
