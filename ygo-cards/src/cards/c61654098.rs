//! World Legacy Discovery
use super::prelude::*;

pub const ID: u32 = 61654098;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_WORLD_CHALICE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // boost
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_WORLD_CHALICE))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_WORLD_CHALICE as u16]))));
    // e2:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    // e2:SetValue(300)
    effect::set_value(d, e2, 300);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // special summon
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e4, EVENT_LEAVE_FIELD);
    // e4:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e4:SetRange(LOCATION_FZONE)
    effect::set_range(d, e4, LOCATION_FZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetCondition(s.spcon)
    effect::set_condition(d, e4, Some(condition(spcon)));
    // e4:SetTarget(s.sptg)
    effect::set_target(d, e4, Some(cost_target(sptg)));
    // e4:SetOperation(s.spop)
    effect::set_operation(d, e4, Some(operation(spop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.spcfilter(c,tp,rp)`
fn spcfilter(d: &mut Duel, c: CardId, tp: u8, rp: u8) -> bool {
    // return c:IsPreviousPosition(POS_FACEUP) and c:IsPreviousControler(tp) and c:IsPreviousSetCard(SET_WORLD_CHALICE)
    //   and (c:IsReason(REASON_BATTLE) or (rp~=tp and c:IsReason(REASON_EFFECT)))
    card::is_previous_position(d, c, POS_FACEUP)
        && card::is_previous_controler(d, c, tp)
        && card::is_previous_set_card(d, c, &[SET_WORLD_CHALICE as u16])
        && (card::is_reason(d, c, REASON_BATTLE) || (rp != tp && card::is_reason(d, c, REASON_EFFECT)))
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.spcfilter,1,nil,tp,rp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| spcfilter(d, c, a.tp, a.rp), 1, NO_EX)
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_WORLD_CHALICE) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    card::is_set_card(d, c, &[SET_WORLD_CHALICE as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.filter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc, a.e, a.tp);
    }
    if a.chk == Some(0) {
        // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,0,1,nil,e,tp)
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
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
}
