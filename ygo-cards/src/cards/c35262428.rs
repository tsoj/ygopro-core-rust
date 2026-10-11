//! Wiseman's Chalice
use super::prelude::*;

pub const ID: u32 = 35262428;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon 1 monster from opponent's GY to your field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
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
    // return Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)==0
    duel::get_field_group_count(d, a.tp, LOCATION_MZONE, 0) == 0
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(1-tp) and s.filter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, 1 - a.tp) && filter(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.filter,tp,0,LOCATION_GRAVE,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, 0, LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,0,LOCATION_GRAVE,1,1,nil,e,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, 0, LOCATION_GRAVE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)~=0 then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) != 0 {
            // --Cannot be tributed
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetDescription(3303)
            effect::set_description(d, e1, 3303);
            // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UNRELEASABLE_SUM)
            effect::set_code(d, e1, EFFECT_UNRELEASABLE_SUM);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // e1:SetValue(1)
            effect::set_value(d, e1, 1);
            // tc:RegisterEffect(e1,true)
            card::register_effect(d, tc, e1, true);
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetCode(EFFECT_UNRELEASABLE_NONSUM)
            effect::set_code(d, e2, EFFECT_UNRELEASABLE_NONSUM);
            // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
            // e2:SetValue(1)
            effect::set_value(d, e2, 1);
            // tc:RegisterEffect(e2,true)
            card::register_effect(d, tc, e2, true);
            // --Cannot be used as synchro material
            // local e3=Effect.CreateEffect(c)
            let e3 = effect::create_effect(d, c);
            // e3:SetDescription(3310)
            effect::set_description(d, e3, 3310);
            // e3:SetProperty(EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e3, EFFECT_FLAG_CLIENT_HINT, 0);
            // e3:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
            // e3:SetCode(EFFECT_CANNOT_BE_SYNCHRO_MATERIAL)
            effect::set_code(d, e3, EFFECT_CANNOT_BE_SYNCHRO_MATERIAL);
            // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
            // e3:SetValue(1)
            effect::set_value(d, e3, 1);
            // tc:RegisterEffect(e3,true)
            card::register_effect(d, tc, e3, true);
            // --Give control of it to your opponent
            // local e4=Effect.CreateEffect(c)
            let e4 = effect::create_effect(d, c);
            // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            // e4:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
            effect::set_property(d, e4, EFFECT_FLAG_IGNORE_IMMUNE, 0);
            // e4:SetRange(LOCATION_MZONE)
            effect::set_range(d, e4, LOCATION_MZONE);
            // e4:SetCode(EVENT_PHASE+PHASE_END)
            effect::set_code(d, e4, EVENT_PHASE + PHASE_END);
            // e4:SetOperation(s.ctlop)
            effect::set_operation(d, e4, Some(operation(ctlop)));
            // e4:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e4, RESETS_STANDARD_PHASE_END, 1);
            // e4:SetCountLimit(1)
            effect::set_count_limit(d, e4, 1, 0, 0, 0);
            // e4:SetLabel(1-tp)
            effect::set_label(d, e4, &[(1 - a.tp) as i64]);
            // tc:RegisterEffect(e4,true)
            card::register_effect(d, tc, e4, true);
        }
    }
}
/// `s.ctlop(e,tp,eg,ep,ev,re,r,rp)`
fn ctlop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetHandler()
    let tc = effect::get_handler(d, a.e).nil();
    // local p=e:GetLabel()
    let p = effect::get_label(d, a.e) as u8;
    // Duel.GetControl(tc,p)
    duel::get_control(d, tc, p, 0, 0, None, None);
}
