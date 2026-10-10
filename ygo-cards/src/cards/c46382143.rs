//! Numeron Creation
use super::prelude::*;

pub const ID: u32 = 46382143;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GALAXY_EYES, SET_NUMBER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "Number" Dragon Xyz Monster from your Extra Deck, then attach this card on the field to it as material
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
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
/// `s.confilter(c)`
fn confilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsRace(RACE_DRAGON) and c:GetBaseAttack()>=3000 and c:IsFaceup()
    card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_race(d, c, RACE_DRAGON) && card::get_base_attack(d, c) >= 3000 && card::is_faceup(d, c)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.confilter,tp,LOCATION_MZONE,LOCATION_MZONE,3,nil)
    duel::is_existing_matching_card(d, Some(&confilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 3, NO_EX)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_NUMBER) and c:IsRace(RACE_DRAGON) and c:IsType(TYPE_XYZ) and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_NUMBER as u16])
        && card::is_race(d, c, RACE_DRAGON)
        && card::is_type(d, c, TYPE_XYZ)
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return not Duel.HasFlagEffect(tp,id)
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return !duel::has_flag_effect(d, a.tp, ID, None)
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.HasFlagEffect(tp,id) then return end
    if duel::has_flag_effect(d, a.tp, ID, None) {
        return;
    }
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if sc and Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)>0 and c:IsRelateToEffect(e) then
    if let Some(sc) = sc {
        if duel::special_summon(d, sc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 && card::is_relate_to_effect(d, c, a.e) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // c:CancelToGrave()
            card::cancel_to_grave(d, c, true);
            // Duel.Overlay(sc,c)
            duel::overlay(d, sc, c, false);
        }
    }
}
