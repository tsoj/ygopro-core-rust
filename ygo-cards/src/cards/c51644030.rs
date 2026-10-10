//! Diabound Kernel
use super::prelude::*;

pub const ID: u32 = 51644030;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atk up
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetOperation(s.atkop1)
    effect::set_operation(d, e1, Some(operation(atkop1)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // atk down
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE + CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e2, TIMING_DAMAGE_STEP, None);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(aux.StatChangeDamageStepCondition)
    effect::set_condition(d, e2, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    // e2:SetTarget(s.atktg)
    effect::set_target(d, e2, Some(cost_target(atktg)));
    // e2:SetOperation(s.atkop2)
    effect::set_operation(d, e2, Some(operation(atkop2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// s.atkop1(e,tp,eg,ep,ev,re,r,rp)
fn atkop1(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFaceup() and c:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(600)
        effect::set_value(d, e1, 600);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && card::is_faceup(d, chkc);
    }
    // if chk==0 then return e:GetHandler():IsAbleToRemove() and e:GetHandler():GetAttack()>0
    //     and Duel.IsExistingTarget(Card.IsFaceup,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_able_to_remove(d, h, None, None, None)
            && card::get_attack(d, h) > 0
            && duel::is_existing_target(d, Some(&|d, x| card::is_faceup(d, x)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,Card.IsFaceup,tp,0,LOCATION_MZONE,1,1,nil)
    duel::select_target(d, a.tp, Some(&|d, x| card::is_faceup(d, x)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// s.atkop2(e,tp,eg,ep,ev,re,r,rp)
fn atkop2(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() and c:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) && card::is_faceup(d, c) {
        // local atk=c:GetAttack()
        let atk = card::get_attack(d, c);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(-atk)
        effect::set_value(d, e1, -(atk as i64));
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // if c:IsRelateToEffect(e) and not tc:IsHasEffect(EFFECT_REVERSE_UPDATE) then
        if card::is_relate_to_effect(d, c, a.e) && card::is_has_effect(d, tc, EFFECT_REVERSE_UPDATE, None).is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // if Duel.Remove(c,0,REASON_EFFECT|REASON_TEMPORARY)~=0 then
            if duel::remove(d, c, 0, REASON_EFFECT | REASON_TEMPORARY, None, None) != 0 {
                // local e2=Effect.CreateEffect(c)
                let e2 = effect::create_effect(d, c);
                // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
                effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
                // e2:SetCode(EVENT_PHASE|PHASE_STANDBY)
                effect::set_code(d, e2, EVENT_PHASE | PHASE_STANDBY);
                // e2:SetCountLimit(1)
                effect::set_count_limit(d, e2, 1, 0, 0, 0);
                // e2:SetLabel(Duel.GetTurnCount())
                let turn = duel::get_turn_count(d, None);
                effect::set_label(d, e2, &[turn as i64]);
                // e2:SetLabelObject(c)
                effect::set_label_object(d, e2, Value::Card(c));
                // e2:SetCondition(s.retcon)
                effect::set_condition(d, e2, Some(condition(retcon)));
                // e2:SetOperation(s.retop)
                effect::set_operation(d, e2, Some(operation(retop)));
                // if Duel.GetCurrentPhase()<=PHASE_STANDBY then
                if duel::get_current_phase(d) <= PHASE_STANDBY {
                    // e2:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_STANDBY,2)
                    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_STANDBY, 2);
                } else {
                    // e2:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_STANDBY)
                    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_STANDBY, 1);
                }
                // Duel.RegisterEffect(e2,tp)
                duel::register_effect(d, e2, a.tp);
            }
        }
    }
}
/// s.retcon(e,tp,eg,ep,ev,re,r,rp)
fn retcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetTurnCount()>e:GetLabel()
    duel::get_turn_count(d, None) as i64 > effect::get_label(d, a.e)
}
/// s.retop(e,tp,eg,ep,ev,re,r,rp)
fn retop(d: &mut Duel, a: &EffArgs) {
    // Duel.ReturnToField(e:GetLabelObject())
    let c = effect::get_label_object(d, a.e).as_card().nil();
    duel::return_to_field(d, c, None, None);
}
