//! Downbeat
use super::prelude::*;

pub const ID: u32 = 96540;

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
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetLabel(0)
    effect::set_label(d, e1, &[0]);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(100)
    effect::set_label(d, a.e, &[100]);
    // if chk==0 then return true end
    true
}
/// `s.costfilter(c,e,tp)`
fn costfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and c:GetOriginalLevel()>0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,c,e,tp)
    card::is_faceup(d, c)
        && card::get_original_level(d, c) > 0
        && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, Some(c), e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.spfilter(c,tc,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, tc: Option<CardId>, e: EffectId, tp: u8) -> bool {
    // return c:GetOriginalLevel()==tc:GetOriginalLevel()-1
    //     and c:GetOriginalRace()==tc:GetOriginalRace()
    //     and c:GetOriginalAttribute()==tc:GetOriginalAttribute()
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    let clv = card::get_original_level(d, c) as i64;
    let tc = tc.nil();
    if clv != card::get_original_level(d, tc) as i64 - 1 {
        return false;
    }
    if card::get_original_race(d, c) != card::get_original_race(d, tc) {
        return false;
    }
    if card::get_original_attribute(d, c) != card::get_original_attribute(d, tc) {
        return false;
    }
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()~=100 then return false end
        if effect::get_label(d, a.e) != 100 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return e:IsHasType(EFFECT_TYPE_ACTIVATE) and Duel.GetLocationCount(tp,LOCATION_MZONE)>-1
        //     and Duel.CheckReleaseGroupCost(tp,s.costfilter,1,false,nil,nil,e,tp)
        return effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > -1
            && duel::check_release_group_cost(d, tp, Some(&|d, c| costfilter(d, c, a.e, tp)), 1, 1, false, None, NO_EX);
    }
    // e:SetLabel(0)
    effect::set_label(d, a.e, &[0]);
    // local g=Duel.SelectReleaseGroupCost(tp,s.costfilter,1,1,false,nil,nil,e,tp)
    let g = duel::select_release_group_cost(d, tp, Some(&|d, c| costfilter(d, c, a.e, tp)), 1, 1, false, None, NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g.clone(), REASON_COST, None);
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local c=e:GetHandler()
    let _c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,tc,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, tc, a.e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
