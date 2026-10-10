//! Familiar Knight
use super::prelude::*;

pub const ID: u32 = 89731911;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYED);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsLocation(LOCATION_GRAVE) and e:GetHandler():IsReason(REASON_BATTLE)
    let h = effect::get_handler(d, a.e).nil();
    card::is_location(d, h, LOCATION_GRAVE) && card::is_reason(d, h, REASON_BATTLE)
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:GetLevel()==4 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::get_level(d, c) == 4 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND,0,1,nil,e,tp)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, NO_EX)
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local g1=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
        let g1 = duel::select_matching_card(d, a.tp, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        // Duel.SpecialSummonStep(g1:GetFirst(),0,tp,tp,false,false,POS_FACEUP)
        let c1 = group::get_first(d, &g1).nil();
        duel::special_summon_step(d, c1, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
    // if Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.filter,1-tp,LOCATION_HAND,0,1,nil,e,1-tp)
    //     and Duel.SelectYesNo(1-tp,aux.Stringid(id,1)) then
    if duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.e, 1 - a.tp)), 1 - a.tp, LOCATION_HAND, 0, 1, NO_EX)
        && duel::select_yes_no(d, 1 - a.tp, aux::stringid(ID, 1))
    {
        // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_SPSUMMON as u64);
        // local g2=Duel.SelectMatchingCard(1-tp,s.filter,1-tp,LOCATION_HAND,0,1,1,nil,e,1-tp)
        let g2 = duel::select_matching_card(d, 1 - a.tp, Some(&|d, c| filter(d, c, a.e, 1 - a.tp)), 1 - a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        // Duel.SpecialSummonStep(g2:GetFirst(),0,1-tp,1-tp,false,false,POS_FACEUP)
        let c2 = group::get_first(d, &g2).nil();
        duel::special_summon_step(d, c2, 0, 1 - a.tp, 1 - a.tp, false, false, POS_FACEUP as u8, None);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
