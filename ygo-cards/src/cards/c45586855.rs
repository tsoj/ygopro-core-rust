//! Ally of Justice Cyclone Creator
use super::prelude::*;

pub const ID: u32 = 45586855;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // to hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, None, None)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_COST|REASON_DISCARD,nil)
    duel::discard_hand(d, a.tp, Some(&|d, c| card::is_discardable(d, c, None, None)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
// function s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_TUNER)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_TUNER)
}
// function s.rfilter(c)
fn rfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap() and c:IsAbleToHand()
    card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local ct=Duel.GetMatchingGroupCount(s.cfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let ct = duel::get_matching_group_count(d, Some(&cfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.rfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,ct,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&rfilter), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, ct as u32, NO_EX);
    }
    // local rg=Duel.GetMatchingGroup(s.rfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let rg = duel::get_matching_group(d, Some(&rfilter), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,rg,ct,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(rg), ct as u8, 0, 0);
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp,chk)
fn op(d: &mut Duel, a: &EffArgs) {
    // local ct=Duel.GetMatchingGroupCount(s.cfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let ct = duel::get_matching_group_count(d, Some(&cfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // local rg=Duel.GetMatchingGroup(s.rfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let rg = duel::get_matching_group(d, Some(&rfilter), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // if #rg<ct then return end
    if rg.len() < ct {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    // local sg=rg:Select(tp,ct,ct,nil)
    let sg = group::select(d, &rg, a.tp, ct as u16, ct as u16, false, NO_EX).nil();
    // Duel.HintSelection(sg)
    duel::hint_selection(d, sg.clone(), true);
    // Duel.SendtoHand(sg,nil,REASON_EFFECT)
    duel::send_to_hand(d, sg, None, REASON_EFFECT, None);
}
