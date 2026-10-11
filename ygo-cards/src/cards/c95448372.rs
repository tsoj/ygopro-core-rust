//! Gouki Gameface
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 95448372;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GOUKI, SET_DINOWRESTLER, SET_G_GOLEM], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --When this card is activated: You can Special Summon 1 Level 4 or lower EARTH monster (Warrior, Dinosaur, or Cyberse) from your hand in Defense Position
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Special Summon 1 "Dinowrestler" or "G Golem" monster from your Deck or Extra Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(s.deckexspcost)
    effect::set_cost(d, e2, Some(cost_target(deckexspcost)));
    // e2:SetTarget(s.deckexsptg)
    effect::set_target(d, e2, Some(cost_target(deckexsptg)));
    // e2:SetOperation(s.deckexspop)
    effect::set_operation(d, e2, Some(operation(deckexspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
/// `s.handspfilter(c,e,tp)`
fn handspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelBelow(4) and c:IsAttribute(ATTRIBUTE_EARTH) and c:IsRace(RACE_WARRIOR|RACE_DINOSAUR|RACE_CYBERSE)
    //   and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    card::is_level_below(d, c, 4)
        && card::is_attribute(d, c, ATTRIBUTE_EARTH)
        && card::is_race(d, c, RACE_WARRIOR | RACE_DINOSAUR | RACE_CYBERSE)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.handspfilter,tp,LOCATION_HAND,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| handspfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, NO_EX);
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // if #sg>0 then
        if !sg.is_empty() {
            // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
            duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
        }
    }
}
/// `s.deckexcostfilter(c,e,tp)`
fn deckexcostfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLinkAbove(3) and c:IsSetCard(SET_GOUKI)
    //   and Duel.IsExistingMatchingCard(s.deckexspfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,nil,e,tp,c)
    card::is_link_above(d, c, 3)
        && card::is_set_card(d, c, &[SET_GOUKI as u16])
        && duel::is_existing_matching_card(d, Some(&|d, x| deckexspfilter(d, x, e, tp, Some(c))), tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.deckexspfilter(c,e,tp,cost_card)`: `cost_card` is `nil` (None) when the Lua passes no card
fn deckexspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, cost_card: Option<CardId>) -> bool {
    // if not (c:IsSetCard({SET_DINOWRESTLER,SET_G_GOLEM}) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)) then return false end
    if !(card::is_set_card(d, c, &[SET_DINOWRESTLER as u16, SET_G_GOLEM as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)) {
        return false;
    }
    // if c:IsLocation(LOCATION_DECK) then
    if card::is_location(d, c, LOCATION_DECK) {
        // return Duel.GetMZoneCount(tp,cost_card)>0
        let ex = match cost_card {
            Some(cc) => cc.into(),
            None => NO_EX,
        };
        duel::get_m_zone_count(d, tp, ex, None, None, None).0 > 0
    } else if card::is_location(d, c, LOCATION_EXTRA) {
        // return Duel.GetLocationCountFromEx(tp,tp,cost_card,c)>0
        let ex = match cost_card {
            Some(cc) => cc.into(),
            None => NO_EX,
        };
        duel::get_location_count_from_ex(d, tp, Some(tp), ex, duel::ExtraCard::Card(c), None).0 > 0
    } else {
        // (falls off the end: nil in Lua, which is false)
        false
    }
}
/// `s.deckexspcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn deckexspcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.deckexcostfilter,1,false,nil,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, Some(&|d, c| deckexcostfilter(d, c, e, tp)), 1, 1, false, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.deckexcostfilter,1,1,false,nil,nil,e,tp)
    let g = duel::select_release_group_cost(d, tp, Some(&|d, c| deckexcostfilter(d, c, e, tp)), 1, 1, false, None, NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.deckexsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn deckexsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK|LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_DECK | LOCATION_EXTRA) as i32);
    true
}
/// `s.deckexspop(e,tp,eg,ep,ev,re,r,rp)`
fn deckexspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.deckexspfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| deckexspfilter(d, c, e, tp, None)), tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
