//! Major Riot
use super::prelude::*;

pub const ID: u32 = 9074847;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SPECIAL_SUMMON+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_TO_HAND)
    effect::set_code(d, e1, EVENT_TO_HAND);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn confilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(tp) and c:IsPreviousLocation(LOCATION_MZONE)
    card::is_controler(d, c, tp) && card::is_previous_location(d, c, LOCATION_MZONE)
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp~=tp and eg:IsExists(s.confilter,1,nil,tp)
    let tp = a.tp;
    if a.rp == tp {
        return false;
    }
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| confilter(d, c, tp), 1, NO_EX)
}
fn thfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsLocation(LOCATION_HAND) and c:IsControler(tp)
    card::is_location(d, c, LOCATION_HAND) && card::is_controler(d, c, tp)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:GetLevel()<=4 and c:IsSummonableCard() and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::get_level(d, c) <= 4
        && card::is_summonable_card(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToHand,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(Card.IsAbleToHand,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), n, 0, 0);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetMatchingGroup(Card.IsAbleToHand,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // Duel.SendtoHand(g,nil,REASON_EFFECT)
    duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
    // Duel.BreakEffect()
    duel::break_effect(d);
    // local og=Duel.GetOperatedGroup()
    let og = duel::get_operated_group(d);
    // local ct1=og:FilterCount(s.thfilter,nil,tp)
    let ct1 = group::filter_count(d, &og, &|d, c| thfilter(d, c, tp), NO_EX) as u16;
    // local ct2=og:FilterCount(s.thfilter,nil,1-tp)
    let ct2 = group::filter_count(d, &og, &|d, c| thfilter(d, c, 1 - tp), NO_EX) as u16;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g1=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,ct1,ct1,nil,e,tp)
    let g1 = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, ct1, ct1, false, NO_EX).nil();
    // if #g1>0 then
    if !g1.is_empty() {
        // local tc=g1:GetFirst()
        let _ = group::get_first(d, &g1);
        // for tc in aux.Next(g1) do
        for tc in group::iter(d, &g1) {
            // Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEDOWN_DEFENSE)
            duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEDOWN_DEFENSE as u8, None);
        }
    }
    // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_SPSUMMON as u64);
    // local g2=Duel.SelectMatchingCard(1-tp,s.spfilter,1-tp,LOCATION_HAND,0,ct2,ct2,nil,e,1-tp)
    let g2 = duel::select_matching_card(d, 1 - tp, Some(&|d, c| spfilter(d, c, e, 1 - tp)), 1 - tp, LOCATION_HAND, 0, ct2, ct2, false, NO_EX).nil();
    // if #g2>0 then
    if !g2.is_empty() {
        // local tc=g2:GetFirst()
        let _ = group::get_first(d, &g2);
        // for tc in aux.Next(g2) do
        for tc in group::iter(d, &g2) {
            // Duel.SpecialSummonStep(tc,0,1-tp,1-tp,false,false,POS_FACEDOWN_DEFENSE)
            duel::special_summon_step(d, tc, 0, 1 - tp, 1 - tp, false, false, POS_FACEDOWN_DEFENSE as u8, None);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
