//! Escape from the Dark Dimension
use super::prelude::*;

pub const ID: u32 = 31550470;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // When this card leaves the field, destroy that monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetCode(EVENT_LEAVE_FIELD_P)
    effect::set_code(d, e2, EVENT_LEAVE_FIELD_P);
    // e2:SetOperation(function(e) e:SetLabel(e:GetHandler():IsDisabled() and 1 or 0) end)
    effect::set_operation(d, e2, Some(operation(e2op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e3, EVENT_LEAVE_FIELD);
    // e3:SetOperation(s.mondesop)
    effect::set_operation(d, e3, Some(operation(mondesop)));
    // e3:SetLabelObject(e2)
    effect::set_label_object(d, e3, Value::Effect(e2));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // When that monster is destroyed, destroy this card
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e4:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e4, EVENT_LEAVE_FIELD);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCondition(s.selfdescon)
    effect::set_condition(d, e4, Some(condition(selfdescon)));
    // e4:SetOperation(function(e) Duel.Destroy(e:GetHandler(),REASON_EFFECT) end)
    effect::set_operation(d, e4, Some(operation(e4op)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsAttribute(ATTRIBUTE_DARK) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_faceup(d, c)
        && card::is_attribute(d, c, ATTRIBUTE_DARK)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return chkc:IsLocation(LOCATION_REMOVED) and chkc:IsControler(tp) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_REMOVED) && card::is_controler(d, chkc, tp) && spfilter(d, chkc, e, tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_REMOVED,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_REMOVED,0,1,1,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, tp, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and tc:IsRelateToEffect(e)
    //     and Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP_ATTACK) then
    if card::is_relate_to_effect(d, c, a.e)
        && card::is_relate_to_effect(d, tc.nil(), a.e)
        && duel::special_summon_step(d, tc.nil(), 0, tp, tp, false, false, POS_FACEUP_ATTACK as u8, None)
    {
        // c:SetCardTarget(tc)
        card::set_card_target(d, c, tc.nil());
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
}
/// `function(e) e:SetLabel(e:GetHandler():IsDisabled() and 1 or 0) end`
fn e2op(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let v = if card::is_disabled(d, c) { 1 } else { 0 };
    effect::set_label(d, a.e, &[v]);
}
/// `s.mondesop(e,tp,eg,ep,ev,re,r,rp)`
fn mondesop(d: &mut Duel, a: &EffArgs) {
    // if e:GetLabelObject():GetLabel()~=0 then return end
    let e2 = effect::get_label_object(d, a.e).as_effect().nil();
    if effect::get_label(d, e2) != 0 {
        return;
    }
    // local tc=e:GetHandler():GetFirstCardTarget()
    let c = effect::get_handler(d, a.e).nil();
    let tc = card::get_first_card_target(d, c);
    // if tc and tc:IsLocation(LOCATION_MZONE) then
    if let Some(tc) = tc {
        if card::is_location(d, tc, LOCATION_MZONE) {
            // Duel.Destroy(tc,REASON_EFFECT,LOCATION_REMOVED)
            duel::destroy(d, tc, REASON_EFFECT, Some(LOCATION_REMOVED), None);
        }
    }
}
/// `s.selfdescon(e,tp,eg,ep,ev,re,r,rp)`
fn selfdescon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetHandler():GetFirstCardTarget()
    let c = effect::get_handler(d, a.e).nil();
    let Some(tc) = card::get_first_card_target(d, c) else {
        return false;
    };
    // return tc and eg:IsContains(tc) and tc:IsReason(REASON_DESTROY)
    let eg = a.eg.clone().nil();
    group::is_contains(&eg, tc) && card::is_reason(d, tc, REASON_DESTROY)
}
/// `function(e) Duel.Destroy(e:GetHandler(),REASON_EFFECT) end`
fn e4op(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    duel::destroy(d, c, REASON_EFFECT, None, None);
}
