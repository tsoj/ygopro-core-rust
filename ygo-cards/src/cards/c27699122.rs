//! Toon Page-Flip
use super::prelude::*;

pub const ID: u32 = 27699122;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_TOON_WORLD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Reveal 3 toon monsters from deck, special summon 1 of them
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
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetHintTiming(0,TIMING_BATTLE_START|TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some((TIMING_BATTLE_START | TIMING_END_PHASE) as u32));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,15259703),tp,LOCATION_ONFIELD,0,1,nil)
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[CARD_TOON_WORLD])), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsMonster() and c:IsType(TYPE_TOON) and c:IsCanBeSpecialSummoned(e,0,tp,true,false) and not c:IsPublic()
    card::is_monster(d, c)
        && card::is_type(d, c, TYPE_TOON)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
        && !card::is_public(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local rvg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK,0,nil,e,tp)
    let rvg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, NO_EX);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and rvg:GetClassCount(Card.GetCode)>=3 end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && group::get_class_count(d, &rvg, &|d, c| card::get_code(d, c)[0] as i64) >= 3;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,0,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 0, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local rvg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK,0,nil,e,tp)
    let rvg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, NO_EX);
    // local rg=aux.SelectUnselectGroup(rvg,e,tp,3,3,aux.dncheck,1,tp,HINTMSG_CONFIRM)
    let dn = aux::dncheck();
    let rg = aux::select_unselect_group(d, &rvg, Some(e), tp, Some(3), Some(3), Some(&dn), 1, tp, Some(HINTMSG_CONFIRM as u64), None, None, false).unwrap_or_default();
    // Duel.ConfirmCards(1-tp,rg)
    duel::confirm_cards(d, 1 - tp, rg.clone());
    // local tg=rg:RandomSelect(1-tp,1)
    let tg = group::random_select(d, &rg, 1 - tp, 1);
    // local tc=tg:GetFirst()
    let tc = group::get_first(d, &tg);
    // if tc and tc:IsCanBeSpecialSummoned(e,0,tp,true,false) then
    if let Some(tc) = tc {
        if card::is_can_be_special_summoned(d, tc, e, 0, tp, true, false, None, None, None) {
            // Duel.SpecialSummon(tc,0,tp,tp,true,false,POS_FACEUP)
            duel::special_summon(d, tc, 0, tp, tp, true, false, POS_FACEUP as u8, None);
        }
    }
}
