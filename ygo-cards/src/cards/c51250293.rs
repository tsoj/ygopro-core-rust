//! マジェスペクター・ウィンド (Majespecter Wind)
use super::prelude::*;

pub const ID: u32 = 51250293;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MAJESPECTER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "Majespecter" monster
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
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.spcostfilter(c,tp)
fn spcostfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_WIND) and c:IsRace(RACE_SPELLCASTER) and Duel.GetMZoneCount(tp,c)>0
    card::is_attribute(d, c, ATTRIBUTE_WIND)
        && card::is_race(d, c, RACE_SPELLCASTER)
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_MAJESPECTER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_MAJESPECTER as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // local hg_chk=Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil,e,tp)
    let hg_chk = duel::is_existing_matching_card(
        d,
        Some(&|d: &mut Duel, c: CardId| spfilter(d, c, a.e, a.tp)),
        a.tp,
        LOCATION_HAND | LOCATION_GRAVE,
        0,
        1,
        NO_EX,
    );
    // local release_chk=e:IsHasType(EFFECT_TYPE_ACTIVATE) and Duel.CheckReleaseGroupCost(tp,s.spcostfilter,1,false,nil,nil,tp)
    let release_chk = effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE)
        && duel::check_release_group_cost(d, a.tp, Some(&|d: &mut Duel, c: CardId| spcostfilter(d, c, a.tp)), 1, 1, false, None, NO_EX);
    // if chk==0 then return (release_chk or ft>0)
    //     and (hg_chk or (release_chk and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp))) end
    if a.chk == Some(0) {
        return (release_chk || ft > 0)
            && (hg_chk
                || (release_chk
                    && duel::is_existing_matching_card(
                        d,
                        Some(&|d: &mut Duel, c: CardId| spfilter(d, c, a.e, a.tp)),
                        a.tp,
                        LOCATION_DECK,
                        0,
                        1,
                        NO_EX,
                    )));
    }
    // if release_chk and (ft<1 or not hg_chk or Duel.SelectYesNo(tp,aux.Stringid(id,1))) then
    if release_chk && (ft < 1 || !hg_chk || duel::select_yes_no(d, a.tp, aux::stringid(ID, 1))) {
        // local rg=Duel.SelectReleaseGroupCost(tp,s.spcostfilter,1,1,false,nil,nil,tp)
        let rg = duel::select_release_group_cost(d, a.tp, Some(&|d: &mut Duel, c: CardId| spcostfilter(d, c, a.tp)), 1, 1, false, None, NO_EX);
        // Duel.Release(rg,REASON_COST)
        duel::release(d, rg, REASON_COST, None);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_GRAVE | LOCATION_DECK) as i32);
    } else {
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    }
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local _,_,_,_,loc=Duel.GetOperationInfo(0,CATEGORY_SPECIAL_SUMMON)
    let loc = duel::get_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON).nil().op_param as u32;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,loc,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&aux::necro_valley_filter(|d: &mut Duel, c: CardId| spfilter(d, c, a.e, a.tp))),
        a.tp,
        loc,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if #g>0 then
    if g.len() > 0 {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
