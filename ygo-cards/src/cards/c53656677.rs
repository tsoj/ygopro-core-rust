//! Power Frame
use super::prelude::*;

pub const ID: u32 = 53656677;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e1, EVENT_BE_BATTLE_TARGET);
    // e1:SetCost(aux.RemainFieldCost)
    effect::set_cost(d, e1, Some(aux::remain_field_cost()));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (tp, e) = (a.tp, a.e);
    // if chkc then return chkc==Duel.GetAttackTarget() end
    if let Some(chkc) = a.chkc() {
        return Some(chkc) == duel::get_attack_target(d);
    }
    // local a=Duel.GetAttacker()
    let atk_card = duel::get_attacker(d).nil();
    // local d=Duel.GetAttackTarget()
    let dc = duel::get_attack_target(d);
    // if chk==0 then return d and d:IsControler(tp) and d:IsFaceup() and d:IsCanBeEffectTarget(e)
    //     and d:GetAttack()<a:GetAttack() and e:IsHasType(EFFECT_TYPE_ACTIVATE) end
    if a.chk == Some(0) {
        let Some(dc) = dc else { return false };
        return card::is_controler(d, dc, tp)
            && card::is_faceup(d, dc)
            && card::is_can_be_effect_target(d, dc, Some(e))
            && card::get_attack(d, dc) < card::get_attack(d, atk_card)
            && effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE);
    }
    // Duel.SetTargetCard(d)
    if let Some(dc) = dc {
        duel::set_target_card(d, dc);
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let (tp, e) = (a.tp, a.e);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not Duel.NegateAttack() then
    if !duel::negate_attack(d) {
        // c:CancelToGrave(false)
        card::cancel_to_grave(d, c, false);
        // return
        return;
    }
    // if not c:IsRelateToEffect(e) or c:IsStatus(STATUS_LEAVE_CONFIRMED) then return end
    if !card::is_relate_to_effect(d, c, e) || card::is_status(d, c, STATUS_LEAVE_CONFIRMED) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if let Some(tc) = tc.filter(|&tc| card::is_relate_to_effect(d, tc, e) && card::is_faceup(d, tc)) {
        // Duel.Equip(tp,c,tc)
        duel::equip(d, tp, c, tc, true, false);
        // local a=Duel.GetAttacker()
        let atk_card = duel::get_attacker(d).nil();
        // local d=Duel.GetAttackTarget()
        let dc = duel::get_attack_target(d).nil();
        // local atk=a:GetAttack()-d:GetAttack()
        let mut atk = card::get_attack(d, atk_card) - card::get_attack(d, dc);
        // if atk<0 then atk=0 end
        if atk < 0 {
            atk = 0;
        }
        // --Atkup
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_EQUIP)
        effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(atk)
        effect::set_value(d, e1, atk as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // --Equip limit
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_EQUIP_LIMIT)
        effect::set_code(d, e2, EFFECT_EQUIP_LIMIT);
        // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e2:SetValue(s.eqlimit)
        effect::set_value_fn(d, e2, effect_card_value(|d, e, c| (Some(c) == effect::get_label_object(d, e).as_card()) as i64));
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // e2:SetLabelObject(tc)
        effect::set_label_object(d, e2, Value::Card(tc));
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
    } else {
        // c:CancelToGrave(false)
        card::cancel_to_grave(d, c, false);
    }
}
