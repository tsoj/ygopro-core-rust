//! Double Evolution Pill
use super::prelude::*;

pub const ID: u32 = 38179121;

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
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.costfilter1(c)`
fn costfilter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsRace(RACE_DINOSAUR) and c:IsAbleToRemoveAsCost()
    //     and (c:IsLocation(LOCATION_HAND) or aux.SpElimFilter(c,true))
    card::is_monster(d, c)
        && card::is_race(d, c, RACE_DINOSAUR)
        && card::is_able_to_remove_as_cost(d, c, None)
        && (card::is_location(d, c, LOCATION_HAND) || aux::sp_elim_filter(d, c, true, false))
}
/// `s.costfilter2(c)`
fn costfilter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and not c:IsRace(RACE_DINOSAUR) and c:IsAbleToRemoveAsCost()
    //     and (c:IsLocation(LOCATION_HAND) or aux.SpElimFilter(c,true))
    card::is_monster(d, c)
        && !card::is_race(d, c, RACE_DINOSAUR)
        && card::is_able_to_remove_as_cost(d, c, None)
        && (card::is_location(d, c, LOCATION_HAND) || aux::sp_elim_filter(d, c, true, false))
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_DINOSAUR) and c:IsLevelAbove(7) and c:IsMonster() and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    card::is_race(d, c, RACE_DINOSAUR)
        && card::is_level_above(d, c, 7)
        && card::is_monster(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_DECK|LOCATION_HAND,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_DECK | LOCATION_HAND, 0, NO_EX);
    // local tc=nil
    let mut tc: Option<CardId> = None;
    // if #g==1 and g:GetFirst():IsLocation(LOCATION_HAND) then tc=g:GetFirst() end
    if g.len() == 1 {
        if let Some(first) = group::get_first(d, &g) {
            if card::is_location(d, first, LOCATION_HAND) {
                tc = Some(first);
            }
        }
    }
    let ex = |tc: Option<CardId>| -> Exclude { tc.map_or(NO_EX, |t| t.into()) };
    // if chk==0 then return Duel.IsExistingMatchingCard(s.costfilter1,tp,LOCATION_HAND|LOCATION_MZONE|LOCATION_GRAVE,0,1,tc)
    //     and Duel.IsExistingMatchingCard(s.costfilter2,tp,LOCATION_HAND|LOCATION_MZONE|LOCATION_GRAVE,0,1,tc) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&costfilter1), tp, LOCATION_HAND | LOCATION_MZONE | LOCATION_GRAVE, 0, 1, ex(tc))
            && duel::is_existing_matching_card(d, Some(&costfilter2), tp, LOCATION_HAND | LOCATION_MZONE | LOCATION_GRAVE, 0, 1, ex(tc));
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g1=Duel.SelectMatchingCard(tp,s.costfilter1,tp,LOCATION_HAND|LOCATION_MZONE|LOCATION_GRAVE,0,1,1,tc)
    let mut g1 = duel::select_matching_card(d, tp, Some(&costfilter1), tp, LOCATION_HAND | LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, ex(tc)).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g2=Duel.SelectMatchingCard(tp,s.costfilter2,tp,LOCATION_HAND|LOCATION_MZONE|LOCATION_GRAVE,0,1,1,tc)
    let g2 = duel::select_matching_card(d, tp, Some(&costfilter2), tp, LOCATION_HAND | LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, ex(tc)).nil();
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // Duel.Remove(g1,POS_FACEUP,REASON_COST)
    duel::remove(d, g1, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK|LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_DECK | LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK|LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_DECK | LOCATION_HAND) as i32);
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
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK|LOCATION_HAND,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_DECK | LOCATION_HAND, 0, 1, 1, false, NO_EX);
    // if #g>0 then
    if let Some(g) = g {
        if !g.is_empty() {
            // Duel.SpecialSummon(g,0,tp,tp,true,false,POS_FACEUP)
            duel::special_summon(d, g, 0, tp, tp, true, false, POS_FACEUP as u8, None);
        }
    }
}
