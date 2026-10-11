//! Tuner's High
use super::prelude::*;

pub const ID: u32 = 85821180;

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
    // e1:SetLabel(0)
    effect::set_label(d, e1, &[0]);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c,e,tp)`
fn cfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsMonster() and c:IsDiscardable()
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil,c:GetRace(),c:GetAttribute(),c:GetLevel(),e,tp)
    card::is_monster(d, c) && card::is_discardable(d, c, None, None) && {
        let race = card::get_race(d, c);
        let att = card::get_attribute(d, c);
        let lv = card::get_level(d, c) as i64;
        duel::is_existing_matching_card(d, Some(&|d, x| filter(d, x, race, att, lv, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX)
    }
}
/// `s.filter(c,race,att,lv,e,tp)`
fn filter(d: &mut Duel, c: CardId, race: u64, att: u32, lv: i64, e: EffectId, tp: u8) -> bool {
    // return c:IsType(TYPE_TUNER) and c:IsRace(race) and c:IsAttribute(att) and c:GetLevel()==lv+1 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_type(d, c, TYPE_TUNER)
        && card::is_race(d, c, race)
        && card::is_attribute(d, c, att)
        && card::get_level(d, c) as i64 == lv + 1
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(100)
    effect::set_label(d, a.e, &[100]);
    // return true
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()~=100 or Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return false end
        if effect::get_label(d, e) != 100 || duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return false;
        }
        // return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_HAND,0,1,nil,e,tp)
        return duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DISCARD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DISCARD as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| cfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g).nil();
    // e:SetLabel(tc:GetRace())
    let race = card::get_race(d, tc);
    effect::set_label(d, e, &[race as i64]);
    // e:SetValue(tc:GetAttribute())
    let att = card::get_attribute(d, tc);
    effect::set_value(d, e, att as i64);
    // Duel.SetTargetParam(tc:GetLevel())
    let lv = card::get_level(d, tc);
    duel::set_target_param(d, lv);
    // Duel.SendtoGrave(g,REASON_COST|REASON_DISCARD)
    duel::send_to_grave(d, g, REASON_COST | REASON_DISCARD, None, None);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local race=e:GetLabel()
    let race = effect::get_label(d, e) as u64;
    // local att=e:GetValue()
    let att = effect::get_value(d, e).int() as u32;
    // local lv=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let lv = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil,race,att,lv,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| filter(d, c, race, att, lv, e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
