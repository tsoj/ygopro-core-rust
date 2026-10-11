//! Dimension Dice
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 47292920;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Tribute 1 monster and Special Summon 1 monster from the hand or deck with a die roll effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.dicefilter(c)`: `return c.roll_dice and c:IsFaceup()`
fn dicefilter(d: &mut Duel, c: CardId) -> bool {
    crate::card_info(d, c).roll_dice && card::is_faceup(d, c)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.dicefilter,tp,LOCATION_ONFIELD,0,1,nil)
    duel::is_existing_matching_card(d, Some(&dicefilter), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
/// `s.spcfilter(c,tp)`: `return Duel.GetMZoneCount(tp,c)>0`
fn spcfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.spcfilter,1,false,nil,nil,tp) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, Some(&|d, c| spcfilter(d, c, tp)), 1, 1, false, None, NO_EX);
    }
    // local sg=Duel.SelectReleaseGroupCost(tp,s.spcfilter,1,1,false,nil,nil,tp)
    let sg = duel::select_release_group_cost(d, tp, Some(&|d, c| spcfilter(d, c, tp)), 1, 1, false, None, NO_EX);
    // Duel.Release(sg,REASON_COST)
    duel::release(d, sg, REASON_COST, None);
    true
}
/// `s.spfilter(c,e,tp)`
/// pending rulings, cannot SS Normal Pendulums with a die roll Pendulum Effect
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c.roll_dice and c:IsCanBeSpecialSummoned(e,0,tp,false,false) and c:IsType(TYPE_EFFECT)
    crate::card_info(d, c).roll_dice
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && card::is_type(d, c, TYPE_EFFECT)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
