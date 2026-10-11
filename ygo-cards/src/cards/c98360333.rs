//! Evil★Twin Challenge
use super::prelude::*;

pub const ID: u32 = 98360333;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    listed_series: &[SET_KI_SIKIL as u32, SET_LIL_LA as u32],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon 1 “Kisikil” or “Lilla” monster from GY
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
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMING_END_PHASE));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// s.filter(c,e,tp)
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return (c:IsSetCard(SET_KI_SIKIL) or c:IsSetCard(SET_LIL_LA)) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP)
    (card::is_set_card(d, c, &[SET_KI_SIKIL as u16]) || card::is_set_card(d, c, &[SET_LIL_LA as u16]))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), None, None)
}
// Check for "Evil★Twin" link monster
fn lkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_EVIL_TWIN) and c:IsLinkSummonable()
    card::is_set_card(d, c, &[SET_EVIL_TWIN as u16]) && card::is_link_summonable(d, c, None, None, 0, 0)
}
// Activation legality
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.filter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_GRAVE) && filter(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, 0, 0);
    true
}
// Special summon 1 “Kisikil” or “Lilla” monster from GY
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)~=0 then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) != 0 {
            // Link summon into a "Evil★Twin" monster
            // local g=Duel.GetMatchingGroup(s.lkfilter,tp,LOCATION_EXTRA,0,nil,e,tp)
            let g = duel::get_matching_group(d, Some(&lkfilter), a.tp, LOCATION_EXTRA, 0, NO_EX);
            // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
            if group::get_count(&g) > 0 && duel::select_yes_no(d, a.tp, aux::stringid(ID, 0)) {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
                duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
                // local sg=g:Select(tp,1,1,nil)
                let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
                // Duel.LinkSummon(tp,sg:GetFirst())
                let sc = group::get_first(d, &sg).nil();
                duel::link_summon(d, a.tp, sc, None, None, 0, 0);
            }
        }
    }
}
