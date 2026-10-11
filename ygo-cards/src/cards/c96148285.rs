//! Triggered Summon
use super::prelude::*;

pub const ID: u32 = 96148285;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(Card.IsControler,1,nil,1-tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| card::is_controler(d, c, 1 - a.tp), 1, NO_EX)
}
// s.filter(c,e,tp)
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelBelow(4) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_level_below(d, c, 4) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_HAND,0,nil,e,tp)
        let g = duel::get_matching_group(d, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_HAND, 0, NO_EX);
        // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
        if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local tc=g:Select(tp,1,1,nil):GetFirst()
            let sel = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
            let tc = group::get_first(d, &sel);
            // Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon_step(d, tc.nil(), 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
    // if Duel.GetLocationCount(1-tp,LOCATION_MZONE,1-tp)>0 then
    if duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, Some(1 - tp), None, None).0 > 0 {
        // local g=Duel.GetMatchingGroup(s.filter,1-tp,LOCATION_HAND,0,nil,e,1-tp)
        let g = duel::get_matching_group(d, Some(&|d, c| filter(d, c, e, 1 - tp)), 1 - tp, LOCATION_HAND, 0, NO_EX);
        // if #g>0 and Duel.SelectYesNo(1-tp,aux.Stringid(id,0)) then
        if !g.is_empty() && duel::select_yes_no(d, 1 - tp, aux::stringid(ID, 0)) {
            // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_SPSUMMON as u64);
            // local tc=g:Select(1-tp,1,1,nil):GetFirst()
            let sel = group::select(d, &g, 1 - tp, 1, 1, false, NO_EX).nil();
            let tc = group::get_first(d, &sel);
            // Duel.SpecialSummonStep(tc,0,1-tp,1-tp,false,false,POS_FACEUP)
            duel::special_summon_step(d, tc.nil(), 0, 1 - tp, 1 - tp, false, false, POS_FACEUP as u8, None);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
