//! Labrynth Stovie Torbie
use super::prelude::*;

pub const ID: u32 = 74018812;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_LABRYNTH], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Set 1 "Labrynth" Spell/Trap from the hand or deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_HAND|LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetHintTiming(TIMING_END_PHASE,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, TIMING_END_PHASE, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetCost(s.setcost)
    effect::set_cost(d, e1, Some(cost_target(setcost)));
    // e1:SetOperation(s.setop)
    effect::set_operation(d, e1, Some(operation(setop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon itself
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e2:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e2, EVENT_LEAVE_FIELD);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.setfilter(c)
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_LABRYNTH) and c:IsSpellTrap() and c:IsSSetable()
    card::is_set_card(d, c, &[SET_LABRYNTH as u16]) && card::is_spell_trap(d, c) && card::is_s_setable(d, c, false, None)
}
// function s.setcostfilter(c,tp)
fn setcostfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsDiscardable() and Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,c)
    card::is_discardable(d, c, None, None) && duel::is_existing_matching_card(d, Some(&setfilter), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, c.into())
}
// function s.setcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn setcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToGraveAsCost() and Duel.IsExistingMatchingCard(s.setcostfilter,tp,LOCATION_HAND,0,1,c,tp) end
    if a.chk == Some(0) {
        return card::is_able_to_grave_as_cost(d, c)
            && duel::is_existing_matching_card(d, Some(&|d, x| setcostfilter(d, x, a.tp)), a.tp, LOCATION_HAND, 0, 1, c.into());
    }
    // Duel.SendtoGrave(c,REASON_COST)
    duel::send_to_grave(d, CardSet::from_card(c), REASON_COST, None, None);
    // Duel.DiscardHand(tp,s.setcostfilter,1,1,REASON_COST|REASON_DISCARD,nil,tp)
    duel::discard_hand(d, a.tp, Some(&|d, x| setcostfilter(d, x, a.tp)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
// function s.setop(e,tp,eg,ep,ev,re,r,rp)
fn setop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
    // local g=Duel.SelectMatchingCard(tp,s.setfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&setfilter), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SSet(tp,g)
        duel::s_set(d, a.tp, g, None, true);
    }
}
// function s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:IsReason(REASON_EFFECT)
    card::is_previous_location(d, c, LOCATION_MZONE) && card::is_reason(d, c, REASON_EFFECT)
}
// function s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==tp and re and re:IsTrapEffect() and re:GetHandler():GetOriginalType()==TYPE_TRAP
    //     and eg:IsExists(s.cfilter,1,nil) and not eg:IsContains(e:GetHandler())
    if a.rp != a.tp {
        return false;
    }
    let Some(re) = a.re else { return false };
    if !effect::is_trap_effect(d, re) {
        return false;
    }
    let rc = effect::get_handler(d, re).nil();
    if card::get_original_type(d, rc) != TYPE_TRAP {
        return false;
    }
    let eg = a.eg.clone().nil();
    let h = effect::get_handler(d, a.e).nil();
    group::is_exists(d, &eg, &cfilter, 1, NO_EX) && !group::is_contains(&eg, h)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, CardSet::from_card(c), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
