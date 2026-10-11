//! Ghoti Chain
use super::prelude::*;

pub const ID: u32 = 45792753;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GHOTI], ..ScriptInfo::DEFAULT };

// local LOCATION_REMOVED_HAND_DECK_GRAVE=LOCATION_REMOVED|LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE
const LOCATION_REMOVED_HAND_DECK_GRAVE: u32 = LOCATION_REMOVED | LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE;

pub fn initial_effect(d: &mut Duel, c: CardId) {
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
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E|TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E | TIMING_MAIN_END));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(s.spcost)
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.spcostfilter(c,e,tp)
fn spcostfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_FISH) and c:IsAbleToRemoveAsCost() and Duel.GetMZoneCount(tp,c)>0
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_REMOVED_HAND_DECK_GRAVE,0,1,nil,e,tp,c:GetOriginalCode())
    card::is_faceup(d, c)
        && card::is_race(d, c, RACE_FISH)
        && card::is_able_to_remove_as_cost(d, c, None)
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
        && {
            let code = card::get_original_code(d, c);
            duel::is_existing_matching_card(d, Some(&|d, cc| spfilter(d, cc, e, tp, code)), tp, LOCATION_REMOVED_HAND_DECK_GRAVE, 0, 1, NO_EX)
        }
}
// function s.spfilter(c,e,tp,code)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, code: u32) -> bool {
    // return c:IsSetCard(SET_GHOTI) and (c:IsFaceup() or not c:IsLocation(LOCATION_REMOVED))
    //   and not c:IsOriginalCode(code) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_GHOTI as u16])
        && (card::is_faceup(d, c) || !card::is_location(d, c, LOCATION_REMOVED))
        && !card::is_original_code(d, c, &[code])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spcostfilter,tp,LOCATION_MZONE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| spcostfilter(d, c, a.e, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spcostfilter,tp,LOCATION_MZONE,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spcostfilter(d, c, a.e, a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g.clone(), POS_FACEUP as u8, REASON_COST, None, None);
    // e:SetLabel(g:GetFirst():GetOriginalCode())
    let first = group::get_first(d, &g).nil();
    let code = card::get_original_code(d, first);
    effect::set_label(d, a.e, &[code as i64]);
    true
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local res=e:GetLabel()~=0
        let res = effect::get_label(d, a.e) != 0;
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return res
        return res;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_REMOVED_HAND_DECK_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_REMOVED_HAND_DECK_GRAVE as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)==0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_REMOVED_HAND_DECK_GRAVE,0,1,1,nil,e,tp,e:GetLabel()):GetFirst()
    let label = effect::get_label(d, a.e) as u32;
    let (e, tp) = (a.e, a.tp);
    let nv = aux::necro_valley_filter(move |d: &mut Duel, c: CardId| spfilter(d, c, e, tp, label));
    let g = duel::select_matching_card(d, tp, Some(&nv), tp, LOCATION_REMOVED_HAND_DECK_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if tc and Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)>0 then
    if let Some(tc) = tc {
        if duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetDescription(3300)
            effect::set_description(d, e1, 3300);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
            effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
            // e1:SetValue(LOCATION_REMOVED)
            effect::set_value(d, e1, LOCATION_REMOVED as i64);
            // e1:SetReset(RESET_EVENT|RESETS_REDIRECT)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT, 1);
            // tc:RegisterEffect(e1,true)
            card::register_effect(d, tc, e1, true);
        }
    }
}
