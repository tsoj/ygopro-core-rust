//! Burst Breath
use super::prelude::*;

pub const ID: u32 = 80163754;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE | TIMINGS_CHECK_MONSTER));
    // e1:SetLabel(1)
    effect::set_label(d, e1, &[1]);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `aux.FaceupFilter(Card.IsDefenseBelow,def)`
fn faceup_def_below(d: &mut Duel, c: CardId, def: i64) -> bool {
    card::is_faceup(d, c) && card::is_defense_below(d, c, def as i32)
}
/// `s.cfilter(c,def)`
fn cfilter(d: &mut Duel, c: CardId, def: i64) -> bool {
    // return c:IsRace(RACE_DRAGON) and c:IsAttackAbove(def)
    if !(card::is_race(d, c, RACE_DRAGON) && card::is_attack_above(d, c, def as i32)) {
        return false;
    }
    // and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsDefenseBelow,c:GetAttack()),0,LOCATION_MZONE,LOCATION_MZONE,1,c)
    let atk = card::get_attack(d, c) as i64;
    duel::is_existing_matching_card(d, Some(&|d, sc| faceup_def_below(d, sc, atk)), 0, LOCATION_MZONE, LOCATION_MZONE, 1, c.into())
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if #g==0 then return false end
    if g.is_empty() {
        return false;
    }
    // local mg,mdef=g:GetMinGroup(Card.GetDefense)
    let (_mg, mdef) = group::get_min_group(d, &g, &|d, c| card::get_defense(d, c) as i64).nil();
    // if chk==0 then
    if a.chk == Some(0) {
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return Duel.CheckReleaseGroupCost(tp,s.cfilter,1,false,nil,nil,mdef)
        return duel::check_release_group_cost(d, tp, Some(&|d, c| cfilter(d, c, mdef)), 1, 1, false, None, NO_EX);
    }
    // local rg=Duel.SelectReleaseGroupCost(tp,s.cfilter,1,1,false,nil,nil,mdef)
    let rg = duel::select_release_group_cost(d, tp, Some(&|d, c| cfilter(d, c, mdef)), 1, 1, false, None, NO_EX);
    // e:SetLabel(rg:GetFirst():GetAttack())
    let first = group::get_first(d, &rg).nil();
    let atk = card::get_attack(d, first);
    effect::set_label(d, a.e, &[atk as i64]);
    // Duel.Release(rg,REASON_COST)
    duel::release(d, rg, REASON_COST, None);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetLabel()==0 end
    if a.chk == Some(0) {
        return effect::get_label(d, a.e) == 0;
    }
    // local dg=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsDefenseBelow,e:GetLabel()),tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let label = effect::get_label(d, a.e);
    let dg = duel::get_matching_group(d, Some(&|d, c| faceup_def_below(d, c, label)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,dg,#dg,0,0)
    let n = dg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg), n, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local dg=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsDefenseBelow,e:GetLabel()),tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let label = effect::get_label(d, a.e);
    let dg = duel::get_matching_group(d, Some(&|d, c| faceup_def_below(d, c, label)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.Destroy(dg,REASON_EFFECT)
    duel::destroy(d, dg, REASON_EFFECT, None, None);
}
