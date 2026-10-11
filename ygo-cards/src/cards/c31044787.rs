//! Ferret Flames
use super::prelude::*;

pub const ID: u32 = 31044787;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Make the opponent shuffle monsters into the Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_TODECK);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.filter(c,tp)
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:GetAttack()>0 and Duel.IsPlayerCanSendtoDeck(tp,c)
    card::is_faceup(d, c) && card::get_attack(d, c) > 0 && duel::is_player_can_send_to_deck(d, tp, Some(c))
}
// the group of the opponent's monsters that can be shuffled: Duel.GetMatchingGroup(s.filter,tp,0,LOCATION_MZONE,nil,1-tp)
fn opp_group(d: &mut Duel, tp: u8) -> CardSet {
    duel::get_matching_group(d, Some(&|d, c| filter(d, c, 1 - tp)), tp, 0, LOCATION_MZONE, NO_EX)
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(s.filter,tp,0,LOCATION_MZONE,nil,1-tp)
    let g = opp_group(d, a.tp);
    // local atk=g:GetSum(Card.GetAttack)
    let atk = group::get_sum(d, &g, &|d, c| card::get_attack(d, c) as i64);
    // return atk>Duel.GetLP(tp)
    atk > duel::get_lp(d, a.tp) as i64
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,1-tp,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, 1 - a.tp, LOCATION_MZONE as i32);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.filter,tp,0,LOCATION_MZONE,nil,1-tp)
    let g = opp_group(d, tp);
    // local diff=g:GetSum(Card.GetAttack)-Duel.GetLP(tp)
    let diff = group::get_sum(d, &g, &|d, c| card::get_attack(d, c) as i64) - duel::get_lp(d, tp) as i64;
    // if diff<=0 then return end
    if diff <= 0 {
        return;
    }
    // local sg=g:SelectWithSumGreater(1-tp,Card.GetAttack,diff)
    let sg = group::select_with_sum_greater(d, &g, 1 - tp, &|d, c| card::get_attack(d, c) as i64, diff as u32).nil();
    // Duel.SendtoDeck(sg,nil,SEQ_DECKSHUFFLE,REASON_RULE,1-tp)
    duel::send_to_deck(d, sg, None, SEQ_DECKSHUFFLE as i32, REASON_RULE, Some(1 - tp));
}
