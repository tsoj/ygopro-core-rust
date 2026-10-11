//! Core Reinforcement
use super::prelude::*;

pub const ID: u32 = 8057630;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    // e2:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e2, EVENT_LEAVE_FIELD);
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destroy2
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e3, EVENT_LEAVE_FIELD);
    // e3:SetCondition(s.descon2)
    effect::set_condition(d, e3, Some(condition(descon2)));
    // e3:SetOperation(s.desop2)
    effect::set_operation(d, e3, Some(operation(desop2)));
    // e2:SetLabelObject(e3)
    effect::set_label_object(d, e2, Value::Effect(e3));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_KOAKI_MEIRU) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_KOAKI_MEIRU as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.filter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and tc:IsRelateToEffect(e)
    //     and Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP_ATTACK) then
    if card::is_relate_to_effect(d, c, a.e)
        && card::is_relate_to_effect(d, tc.nil(), a.e)
        && duel::special_summon_step(d, tc.nil(), 0, a.tp, a.tp, false, false, POS_FACEUP_ATTACK as u8, None)
    {
        // c:SetCardTarget(tc)
        card::set_card_target(d, c, tc.nil());
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetHandler():GetFirstCardTarget()
    let c = effect::get_handler(d, a.e).nil();
    // if not tc then return end
    let Some(tc) = card::get_first_card_target(d, c) else {
        return;
    };
    // if tc:IsLocation(LOCATION_MZONE) and Duel.Destroy(tc,REASON_EFFECT)==0 then return end
    if card::is_location(d, tc, LOCATION_MZONE) && duel::destroy(d, tc, REASON_EFFECT, None, None) == 0 {
        return;
    }
    // if re~=e:GetLabelObject() and Duel.IsTurnPlayer(tp) and Duel.IsPhase(PHASE_END) then
    let label = effect::get_label_object(d, a.e).as_effect();
    if a.re != label && duel::is_turn_player(d, a.tp) && duel::is_phase(d, PHASE_END) {
        // local atk=tc:GetBaseAttack()
        let mut atk = card::get_base_attack(d, tc);
        // if atk<0 then atk=0 end
        if atk < 0 {
            atk = 0;
        }
        // Duel.Damage(tp,atk,REASON_EFFECT)
        duel::damage(d, a.tp, atk as i64, REASON_EFFECT, false, None);
    }
}
/// `s.descon2(e,tp,eg,ep,ev,re,r,rp)`
fn descon2(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetHandler():GetFirstCardTarget()
    let c = effect::get_handler(d, a.e).nil();
    // return tc and eg:IsContains(tc) and tc:IsReason(REASON_DESTROY)
    let Some(tc) = card::get_first_card_target(d, c) else {
        return false;
    };
    let eg = a.eg.clone().nil();
    group::is_contains(&eg, tc) && card::is_reason(d, tc, REASON_DESTROY)
}
/// `s.desop2(e,tp,eg,ep,ev,re,r,rp)`
fn desop2(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetHandler():GetFirstCardTarget()
    let c = effect::get_handler(d, a.e).nil();
    let tc = card::get_first_card_target(d, c);
    // if Duel.IsTurnPlayer(tp) and Duel.IsPhase(PHASE_END) then
    if duel::is_turn_player(d, a.tp) && duel::is_phase(d, PHASE_END) {
        // local atk=tc:GetBaseAttack()
        let mut atk = card::get_base_attack(d, tc.nil());
        // if atk<0 then atk=0 end
        if atk < 0 {
            atk = 0;
        }
        // Duel.Damage(tp,atk,REASON_EFFECT)
        duel::damage(d, a.tp, atk as i64, REASON_EFFECT, false, None);
    }
    // Duel.Destroy(e:GetHandler(),REASON_EFFECT)
    duel::destroy(d, c, REASON_EFFECT, None, None);
}
