//! United Front
use super::prelude::*;

pub const ID: u32 = 86474024;

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
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.filter1(c,tp)
fn filter1(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // local lv1=c:GetLevel()
    let lv1 = card::get_level(d, c);
    // return lv1>0 and c:IsFaceup() and Duel.IsExistingMatchingCard(s.filter2,tp,LOCATION_MZONE,0,1,c,lv1)
    lv1 > 0 && card::is_faceup(d, c) && duel::is_existing_matching_card(d, Some(&|d, x| filter2(d, x, lv1)), tp, LOCATION_MZONE, 0, 1, c.into())
}
// function s.filter2(c,lv1)
fn filter2(d: &mut Duel, c: CardId, lv1: u32) -> bool {
    // return c:IsFaceup() and c:GetLevel()==lv1
    card::is_faceup(d, c) && card::get_level(d, c) == lv1
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    // return re:IsTrapEffect() and re:IsHasType(EFFECT_TYPE_ACTIVATE) and Duel.IsChainNegatable(ev)
    //     and Duel.IsExistingMatchingCard(s.filter1,tp,LOCATION_MZONE,0,1,nil,tp)
    effect::is_trap_effect(d, re)
        && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
        && duel::is_chain_negatable(d, a.ev as u8)
        && duel::is_existing_matching_card(d, Some(&|d, c| filter1(d, c, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    let re = a.re.nil();
    // if re:GetHandler():IsDestructable() and re:GetHandler():IsRelateToEffect(re) then
    let rh = effect::get_handler(d, re).nil();
    if card::is_destructable(d, rh, None) && card::is_relate_to_effect(d, rh, re) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let re = a.re.nil();
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) {
        let rh = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rh, re) {
            // Duel.Destroy(eg,REASON_EFFECT)
            duel::destroy(d, a.eg.clone().nil(), REASON_EFFECT, None, None);
        }
    }
}
