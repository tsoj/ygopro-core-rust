//! Ogdoadic Calling
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 23837054;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[23837055], listed_series: &[SET_OGDOADIC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_REPTILE) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_REPTILE as u64) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.setfitler(c)
fn setfitler(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_OGDOADIC) and c:IsMonster()
    card::is_set_card(d, c, &[SET_OGDOADIC as u16]) && card::is_monster(d, c)
}
// The Ogdoadic token data: (tp, id+1, SET_OGDOADIC, TYPES_TOKEN, 0, 0, 2, RACE_REPTILE, ATTRIBUTE_DARK)
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![SET_OGDOADIC as u16]),
        type_: Some(TYPES_TOKEN),
        attack: Some(0),
        defense: Some(0),
        level: Some(2),
        race: Some(RACE_REPTILE as u64),
        attribute: Some(ATTRIBUTE_DARK),
    }
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local sg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_GRAVE,0,nil,e,tp)
        let sg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, NO_EX);
        // local g=Duel.GetMatchingGroup(s.setfitler,tp,LOCATION_GRAVE,0,nil)
        let g = duel::get_matching_group(d, Some(&setfitler), a.tp, LOCATION_GRAVE, 0, NO_EX);
        // local gysummon=g:GetClassCount(Card.GetCode)>=8 and sg:GetClassCount(Card.GetCode)>=2
        let gysummon = group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) >= 8 && group::get_class_count(d, &sg, &|d, c| card::get_code(d, c)[0] as i64) >= 2;
        // local tksummon=Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,SET_OGDOADIC,TYPES_TOKEN,0,0,2,RACE_REPTILE,ATTRIBUTE_DARK)
        let tksummon = duel::is_player_can_special_summon_monster(d, a.tp, ID + 1, token_data(), None, None, 0);
        // return not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) and Duel.GetLocationCount(tp,LOCATION_MZONE)>1 and (tksummon or gysummon)
        return duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 1
            && (tksummon || gysummon);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOKEN,nil,2,0,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOKEN, None, 2, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, a.tp, LOCATION_GRAVE as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) or Duel.GetLocationCount(tp,LOCATION_MZONE)<2 then return end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() || duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < 2 {
        return;
    }
    // local sg=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE,0,nil,e,tp)
    let nvf = aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp));
    let sg = duel::get_matching_group(d, Some(&nvf), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // local g=Duel.GetMatchingGroup(s.setfitler,tp,LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&setfitler), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // local gysummon=g:GetClassCount(Card.GetCode)>=8 and sg:GetClassCount(Card.GetCode)>=2
    let gysummon = group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) >= 8 && group::get_class_count(d, &sg, &|d, c| card::get_code(d, c)[0] as i64) >= 2;
    // local tksummon=Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,SET_OGDOADIC,TYPES_TOKEN,0,0,2,RACE_REPTILE,ATTRIBUTE_DARK)
    let tksummon = duel::is_player_can_special_summon_monster(d, a.tp, ID + 1, token_data(), None, None, 0);
    // local choice
    let choice: i32;
    // if tksummon and gysummon then
    if tksummon && gysummon {
        // choice=Duel.SelectOption(tp,aux.Stringid(id,0),aux.Stringid(id,1))
        choice = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 1)]);
    } else if tksummon {
        // elseif tksummon then choice=0
        choice = 0;
    } else if gysummon {
        // elseif gysummon then choice=1
        choice = 1;
    } else {
        // else return end
        return;
    }
    // if choice==0 then
    if choice == 0 {
        // for i=1,2 do
        for _ in 0..2 {
            // local token=Duel.CreateToken(tp,id+1)
            let token = duel::create_token(d, a.tp, ID + 1).nil();
            // Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon_step(d, token, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    } else if choice == 1 {
        // local ssg=aux.SelectUnselectGroup(sg,e,tp,2,2,aux.dncheck,1,tp,HINTMSG_SPSUMMON)
        let ssg = aux::select_unselect_group(d, &sg, Some(a.e), a.tp, Some(2), Some(2), Some(&aux::dncheck()), 1, a.tp, Some(HINTMSG_SPSUMMON as u64), None, None, false).unwrap_or_default();
        // if ssg and #ssg>0 then
        if !ssg.is_empty() {
            // Duel.SpecialSummon(ssg,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, ssg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
