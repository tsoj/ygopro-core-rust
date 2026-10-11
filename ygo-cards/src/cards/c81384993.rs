//! Malefic Divide
use super::prelude::*;

pub const ID: u32 = 81384993;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MALEFIC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    // e1:SetCode(EVENT_FREE_CHAIN)
    // e1:SetTarget(s.target)
    // e1:SetOperation(s.activate)
    // c:RegisterEffect(e1)
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
// function s.filter(c,e,tp)
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_MALEFIC) and c:IsMonster() and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    card::is_set_card(d, c, &[SET_MALEFIC as u16]) && card::is_monster(d, c) && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.filter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && filter(d, chkc, a.e, tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        let e = a.e;
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let e = a.e;
    let g = duel::select_target(d, tp, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, 0, 0);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.SpecialSummonStep(tc,0,tp,tp,true,false,POS_FACEUP) then
    if card::is_relate_to_effect(d, tc, a.e) && duel::special_summon_step(d, tc, 0, tp, tp, true, false, POS_FACEUP as u8, None) {
        // local e1=Effect.CreateEffect(c)
        // e1:SetType(EFFECT_TYPE_SINGLE)
        // e1:SetCode(EFFECT_DISABLE)
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        // tc:RegisterEffect(e1,true)
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_DISABLE);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e1, true);
        // local e2=Effect.CreateEffect(c)
        // e2:SetType(EFFECT_TYPE_SINGLE)
        // e2:SetCode(EFFECT_DISABLE_EFFECT)
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        // tc:RegisterEffect(e2,true)
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e2, true);
        // local fid=c:GetFieldID()
        let fid = card::get_field_id(d, c);
        // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1,fid)
        card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, fid as i64, 0);
        // local e3=Effect.CreateEffect(c)
        // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        // e3:SetCode(EVENT_PHASE+PHASE_END)
        // e3:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
        // e3:SetCountLimit(1)
        // e3:SetLabel(fid)
        // e3:SetLabelObject(tc)
        // e3:SetCondition(s.rmcon)
        // e3:SetOperation(s.rmop)
        // Duel.RegisterEffect(e3,tp)
        let e3 = effect::create_effect(d, c);
        effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
        effect::set_property(d, e3, EFFECT_FLAG_IGNORE_IMMUNE, 0);
        effect::set_count_limit(d, e3, 1, 0, 0, 0);
        effect::set_label(d, e3, &[fid as i64]);
        effect::set_label_object(d, e3, Value::Card(tc));
        effect::set_condition(d, e3, Some(condition(rmcon)));
        effect::set_operation(d, e3, Some(operation(rmop)));
        duel::register_effect(d, e3, tp);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
// function s.rmcon(e,tp,eg,ep,ev,re,r,rp)
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
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
// function s.rmop(e,tp,eg,ep,ev,re,r,rp)
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
    duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
