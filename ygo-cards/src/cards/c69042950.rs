//! Crashbug Road
use super::prelude::*;

pub const ID: u32 = 69042950;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.mfilter(c,clv)`
fn mfilter(d: &mut Duel, c: CardId, clv: u32) -> bool {
    // return c:IsFaceup() and c:GetLevel()==clv
    card::is_faceup(d, c) && card::get_level(d, c) == clv
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // local lv=c:GetLevel()
    let lv = card::get_level(d, c);
    // return lv>0 and lv<=4 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //   and Duel.IsExistingMatchingCard(s.mfilter,tp,LOCATION_MZONE,0,1,nil,lv)
    lv > 0
        && lv <= 4
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && duel::is_existing_matching_card(d, Some(&|d, x| mfilter(d, x, lv)), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp)
    //   and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsLevelBelow,4),tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, x| card::is_level_below(d, x, 4))), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        // if #g~=0 then
        if !g.is_empty() {
            // Duel.SpecialSummonStep(g:GetFirst(),0,tp,tp,false,false,POS_FACEUP)
            let first = group::get_first(d, &g).nil();
            duel::special_summon_step(d, first, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
    // if Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.spfilter,1-tp,LOCATION_HAND,0,1,nil,e,1-tp)
    //   and Duel.SelectYesNo(1-tp,aux.Stringid(id,0)) then
    if duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, 1 - tp)), 1 - tp, LOCATION_HAND, 0, 1, NO_EX)
        && duel::select_yes_no(d, 1 - tp, aux::stringid(ID, 0))
    {
        // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(1-tp,s.spfilter,1-tp,LOCATION_HAND,0,1,1,nil,e,1-tp)
        let g = duel::select_matching_card(d, 1 - tp, Some(&|d, x| spfilter(d, x, e, 1 - tp)), 1 - tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        // if #g~=0 then
        if !g.is_empty() {
            // Duel.SpecialSummonStep(g:GetFirst(),0,1-tp,1-tp,false,false,POS_FACEUP)
            let first = group::get_first(d, &g).nil();
            duel::special_summon_step(d, first, 0, 1 - tp, 1 - tp, false, false, POS_FACEUP as u8, None);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
