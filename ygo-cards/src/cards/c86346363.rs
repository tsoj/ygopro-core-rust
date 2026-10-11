//! Time Reloader
use super::prelude::*;

pub const ID: u32 = 86346363;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_POSITION+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_POSITION + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_FLIP)
    effect::set_code(d, e1, EVENT_FLIP);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e1, Some(condition(main_phase_cond)));
    // e1:SetTarget(s.selfsptg)
    effect::set_target(d, e1, Some(cost_target(selfsptg)));
    // e1:SetOperation(s.selfspop)
    effect::set_operation(d, e1, Some(operation(selfspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_POSITION+CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_POSITION + CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FLIP)
    effect::set_code(d, e2, EVENT_FLIP);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e2, Some(condition(main_phase_cond)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function(e,tp,eg,ep,ev,re,r,rp) return Duel.IsMainPhase() end
fn main_phase_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_main_phase(d, None)
}
// function s.selfsptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn selfsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //   and Duel.IsExistingMatchingCard(Card.IsCanTurnSet,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None)
            && duel::is_existing_matching_card(d, Some(&card::is_can_turn_set), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, LOCATION_HAND as i32);
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,nil,1,PLAYER_EITHER,POS_FACEDOWN_DEFENSE)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, None, 1, PLAYER_EITHER as u8, POS_FACEDOWN_DEFENSE as i32);
    true
}
// function s.selfspop(e,tp,eg,ep,ev,re,r,rp)
fn selfspop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0 then
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
        change_position_select(d, a.tp);
    }
}
// Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE) ... Duel.ChangePosition(g,POS_FACEDOWN_DEFENSE)
fn change_position_select(d: &mut Duel, tp: u8) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_POSCHANGE as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsCanTurnSet,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&card::is_can_turn_set), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // Duel.HintSelection(g,true)
    duel::hint_selection(d, g.clone(), true);
    // Duel.BreakEffect()
    duel::break_effect(d);
    // Duel.ChangePosition(g,POS_FACEDOWN_DEFENSE)
    duel::change_position(d, g, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && spfilter(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp)
    //   and Duel.IsExistingMatchingCard(Card.IsCanTurnSet,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, cc| spfilter(d, cc, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&card::is_can_turn_set), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, cc| spfilter(d, cc, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,nil,1,PLAYER_EITHER,POS_FACEDOWN_DEFENSE)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, None, 1, PLAYER_EITHER as u8, POS_FACEDOWN_DEFENSE as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP_DEFENSE)>0 then
    if card::is_relate_to_effect(d, tc, a.e) && duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None) > 0 {
        change_position_select(d, a.tp);
    }
}
