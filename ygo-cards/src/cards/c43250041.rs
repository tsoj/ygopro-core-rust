//! Draining Shield
use super::prelude::*;

pub const ID: u32 = 43250041;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_RECOVER)
    effect::set_category(d, e1, CATEGORY_RECOVER);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
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
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp)
    duel::is_turn_player(d, 1 - a.tp)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local tg=Duel.GetAttacker()
    let tg = duel::get_attacker(d);
    // if chkc then return chkc==tg end
    if let Some(chkc) = a.chkc() {
        return Some(chkc) == tg;
    }
    // if chk==0 then return tg and tg:IsOnField() and tg:IsCanBeEffectTarget(e) end
    if a.chk == Some(0) {
        return match tg {
            Some(tg) => card::is_on_field(d, tg) && card::is_can_be_effect_target(d, tg, Some(a.e)),
            None => false,
        };
    }
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.nil());
    // local rec=tg:GetAttack()
    let rec = card::get_attack(d, tg.nil());
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,rec)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, rec);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and tc:IsFaceup() and tc:CanAttack() then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) && card::can_attack(d, tc) {
            // if Duel.NegateAttack(tc) then
            if duel::negate_attack(d) {
                // Duel.Recover(tp,tc:GetAttack(),REASON_EFFECT)
                let atk = card::get_attack(d, tc);
                duel::recover(d, a.tp, atk as i64, REASON_EFFECT, false, None);
            }
        }
    }
}
