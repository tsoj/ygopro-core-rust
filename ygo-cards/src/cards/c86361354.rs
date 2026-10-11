//! Tribe-Shocking Virus
use super::prelude::*;

pub const ID: u32 = 86361354;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Once per turn: You can banish 1 monster from your hand; destroy all face-up monsters on the field with the same Type as that monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(s.descost)
    effect::set_cost(d, e1, Some(cost_target(descost)));
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.descostfilter(c)`
fn descostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsAbleToRemoveAsCost()
    //   and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,c:GetRace()),0,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    if !(card::is_monster(d, c) && card::is_able_to_remove_as_cost(d, c, None)) {
        return false;
    }
    let race = card::get_race(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| faceup_race(d, x, race)), 0, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
}
/// `aux.FaceupFilter(Card.IsRace,race)`
fn faceup_race(d: &mut Duel, c: CardId, race: u64) -> bool {
    card::is_faceup(d, c) && card::is_race(d, c, race)
}
/// `s.descost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(-100)
    effect::set_label(d, a.e, &[-100]);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.descostfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&descostfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.descostfilter,tp,LOCATION_HAND,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&descostfilter), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel).nil();
    // e:SetLabel(sc:GetRace())
    let race = card::get_race(d, sc);
    effect::set_label(d, a.e, &[race as i64]);
    // Duel.Remove(sc,POS_FACEUP,REASON_COST)
    duel::remove(d, sc, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local cost_skip_chk=e:GetLabel()==-100
        let cost_skip_chk = effect::get_label(d, a.e) == -100;
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return cost_skip_chk
        return cost_skip_chk;
    }
    // local race=e:GetLabel()
    let race = effect::get_label(d, a.e) as u64;
    // Duel.SetTargetParam(race)
    duel::set_target_param(d, race as u32);
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsRace,race),0,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, x| faceup_race(d, x, race)), 0, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    let count = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), count, a.tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, _a: &EffArgs) {
    // local race=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let race = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int() as u64;
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsRace,race),0,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, x| faceup_race(d, x, race)), 0, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if #g>0 then
    if g.len() > 0 {
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
