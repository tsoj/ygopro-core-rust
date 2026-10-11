//! Ryzeal Plugin
use super::prelude::*;

pub const ID: u32 = 60394026;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_RYZEAL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 Xyz Monster or "Ryzeal" monster from your GY or banishment
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
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return (c:IsType(TYPE_XYZ) or c:IsSetCard(SET_RYZEAL)) and c:IsFaceup() and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    (card::is_type(d, c, TYPE_XYZ) || card::is_set_card(d, c, &[SET_RYZEAL as u16]))
        && card::is_faceup(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE|LOCATION_REMOVED) and chkc:IsControler(tp) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE | LOCATION_REMOVED) && card::is_controler(d, chkc, tp) && spfilter(d, chkc, a.e, tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,1,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, tp, 0);
    true
}
// function s.attachfilter(c,tp)
fn attachfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_RYZEAL) and Duel.IsExistingMatchingCard(s.xyzfilter,tp,LOCATION_MZONE,0,1,nil,c,tp)
    card::is_set_card(d, c, &[SET_RYZEAL as u16]) && duel::is_existing_matching_card(d, Some(&|d, x| xyzfilter(d, x, c, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
// function s.xyzfilter(c,mc,tp)
fn xyzfilter(d: &mut Duel, c: CardId, mc: CardId, tp: u8) -> bool {
    // return c:IsType(TYPE_XYZ) and c:IsRank(4) and c:IsFaceup() and mc:IsCanBeXyzMaterial(c,tp,REASON_EFFECT)
    card::is_type(d, c, TYPE_XYZ) && card::is_rank(d, c, &[4]) && card::is_faceup(d, c) && card::is_can_be_xyz_material(d, mc, Some(c), Some(tp), Some(REASON_EFFECT))
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)>0
    //     and Duel.IsExistingMatchingCard(s.attachfilter,tp,LOCATION_DECK,0,1,nil,tp)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| attachfilter(d, c, tp)), tp, LOCATION_DECK, 0, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATTACH)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATTACH as u64);
        // local mc=Duel.SelectMatchingCard(tp,s.attachfilter,tp,LOCATION_DECK,0,1,1,nil,tp):GetFirst()
        let sel = duel::select_matching_card(d, tp, Some(&|d, c| attachfilter(d, c, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        let mc = group::get_first(d, &sel).nil();
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
        // local xyzc=Duel.SelectMatchingCard(tp,s.xyzfilter,tp,LOCATION_MZONE,0,1,1,nil,mc,tp):GetFirst()
        let sel2 = duel::select_matching_card(d, tp, Some(&|d, x| xyzfilter(d, x, mc, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
        let xyzc = group::get_first(d, &sel2).nil();
        // if not xyzc:IsImmuneToEffect(e) then
        if !card::is_immune_to_effect(d, xyzc, a.e) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Overlay(xyzc,mc)
            duel::overlay(d, xyzc, CardSet::from_card(mc), false);
        }
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // You cannot declare attacks for the rest of this turn, except with Rank 4 Xyz Monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK_ANNOUNCE);
    // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(function(e,c) return not (c:IsType(TYPE_XYZ) and c:IsRank(4)) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| !(card::is_type(d, c, TYPE_XYZ) && card::is_rank(d, c, &[4])))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // aux.RegisterClientHint(c,0,tp,1,0,aux.Stringid(id,2))
    aux::register_client_hint(d, c, 0, tp, 1, 0, Some(aux::stringid(ID, 2)), 0, None);
}
