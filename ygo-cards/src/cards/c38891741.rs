//! Ultimate Providence
use super::prelude::*;

pub const ID: u32 = 38891741;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
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
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return (re:IsMonsterEffect() or re:IsHasType(EFFECT_TYPE_ACTIVATE)) and Duel.IsChainNegatable(ev)
    let re = a.re.nil();
    (effect::is_monster_effect(d, re) || effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)) && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.cfilter(c,type)`
fn cfilter(d: &mut Duel, c: CardId, ty: u32) -> bool {
    // return c:IsType(type) and c:IsDiscardable()
    card::is_type(d, c, ty) && card::is_discardable(d, c, None, None)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if Duel.IsPlayerAffectedByEffect(tp,EFFECT_DISCARD_COST_CHANGE) then return true end
    if duel::is_player_affected_by_effect(d, a.tp, EFFECT_DISCARD_COST_CHANGE).is_some() {
        return true;
    }
    // local type=(re:GetActiveType()&(TYPE_MONSTER|TYPE_SPELL|TYPE_TRAP))
    let re = a.re.nil();
    let ty = effect::get_active_type(d, re) & (TYPE_MONSTER | TYPE_SPELL | TYPE_TRAP);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_HAND,0,1,nil,type) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c, ty)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.DiscardHand(tp,s.cfilter,1,1,REASON_COST|REASON_DISCARD,nil,type)
    duel::discard_hand(d, a.tp, Some(&|d, c| cfilter(d, c, ty)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // if re:GetHandler():IsDestructable() and re:GetHandler():IsRelateToEffect(re) then
    let h = effect::get_handler(d, a.re.nil()).nil();
    if card::is_destructable(d, h, None) && card::is_relate_to_effect(d, h, a.re.nil()) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if not Duel.NegateActivation(ev) then return end
    if !duel::negate_activation(d, a.ev as u8) {
        return;
    }
    // if re:GetHandler():IsRelateToEffect(re) then
    let h = effect::get_handler(d, a.re.nil()).nil();
    if card::is_relate_to_effect(d, h, a.re.nil()) {
        // Duel.Destroy(eg,REASON_EFFECT)
        duel::destroy(d, a.eg.clone().nil(), REASON_EFFECT, None, None);
    }
}
