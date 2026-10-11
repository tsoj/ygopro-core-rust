//! Bending Destiny
use super::prelude::*;

pub const ID: u32 = 94662235;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FORTUNE_LADY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate(summon)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DISABLE_SUMMON+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_DISABLE_SUMMON + CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_SUMMON)
    effect::set_code(d, e1, EVENT_SUMMON);
    // e1:SetCondition(s.condition1)
    effect::set_condition(d, e1, Some(condition(condition1)));
    // e1:SetTarget(s.target1)
    effect::set_target(d, e1, Some(cost_target(target1)));
    // e1:SetOperation(s.activate1)
    effect::set_operation(d, e1, Some(operation(activate1)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Activate(effect)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_NEGATE+CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_NEGATE + CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e2, EFFECT_TYPE_ACTIVATE);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetCondition(s.condition2)
    effect::set_condition(d, e2, Some(condition(condition2)));
    // e2:SetTarget(s.target2)
    effect::set_target(d, e2, Some(cost_target(target2)));
    // e2:SetOperation(s.activate2)
    effect::set_operation(d, e2, Some(operation(activate2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.check(tp)
fn check(d: &mut Duel, tp: u8) -> bool {
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, NO_EX);
    // local count=#g
    let count = g.len();
    // return count>0 and g:FilterCount(Card.IsSetCard,nil,SET_FORTUNE_LADY)==count
    count > 0 && group::filter_count(d, &g, &|d, c| card::is_set_card(d, c, &[SET_FORTUNE_LADY as u16]), NO_EX) == count
}
// function s.condition1(e,tp,eg,ep,ev,re,r,rp)
fn condition1(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetCurrentChain(true)==0 and s.check(tp)
    duel::get_current_chain(d, true) == 0 && check(d, a.tp)
}
// function s.target1(e,tp,eg,ep,ev,re,r,rp,chk)
fn target1(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return eg:GetFirst():IsAbleToRemove() end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        let c = group::get_first(d, &eg).nil();
        return card::is_able_to_remove(d, c, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE_SUMMON,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE_SUMMON, a.eg.clone(), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, a.eg.clone(), 1, 0, 0);
    true
}
// function s.activate1(e,tp,eg,ep,ev,re,r,rp)
fn activate1(d: &mut Duel, a: &EffArgs) {
    // Duel.NegateSummon(eg:GetFirst())
    let eg = a.eg.clone().nil();
    let first = group::get_first(d, &eg).nil();
    duel::negate_summon(d, first);
    // local ec=eg:GetFirst()
    let ec = group::get_first(d, &eg).nil();
    // Duel.Remove(ec,POS_FACEUP,REASON_EFFECT)
    duel::remove(d, ec, POS_FACEUP as u8, REASON_EFFECT, None, None);
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    register_return(d, ec, h);
}
// The return effect of activate1 and activate2 (the same lines in the Lua):
// e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS) ... ec:RegisterEffect(e1)
fn register_return(d: &mut Duel, ec: CardId, h: CardId) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetRange(LOCATION_REMOVED)
    effect::set_range(d, e1, LOCATION_REMOVED);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // e1:SetOperation(s.retop)
    effect::set_operation(d, e1, Some(operation(retop)));
    // ec:RegisterEffect(e1)
    card::register_effect(d, ec, e1, false);
}
// function s.condition2(e,tp,eg,ep,ev,re,r,rp)
fn condition2(d: &mut Duel, a: &EffArgs) -> bool {
    // return re:IsHasType(EFFECT_TYPE_ACTIVATE) and Duel.IsChainNegatable(ev) and s.check(tp)
    let re = a.re.nil();
    effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && duel::is_chain_negatable(d, a.ev as u8) && check(d, a.tp)
}
// function s.target2(e,tp,eg,ep,ev,re,r,rp,chk)
fn target2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return eg:GetFirst():IsAbleToRemove() end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        let c = group::get_first(d, &eg).nil();
        return card::is_able_to_remove(d, c, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // if re:GetHandler():IsRelateToEffect(re) then
    let re = a.re.nil();
    let rh = effect::get_handler(d, re).nil();
    if card::is_relate_to_effect(d, rh, re) {
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, a.eg.clone(), 1, 0, 0);
    }
    true
}
// function s.activate2(e,tp,eg,ep,ev,re,r,rp)
fn activate2(d: &mut Duel, a: &EffArgs) {
    // local ec=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let ec = group::get_first(d, &eg).nil();
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) {
        let re = a.re.nil();
        let rh = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rh, re) {
            // Duel.Remove(ec,POS_FACEUP,REASON_EFFECT)
            duel::remove(d, ec, POS_FACEUP as u8, REASON_EFFECT, None, None);
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            register_return(d, ec, h);
        }
    }
}
// function s.retop(e,tp,eg,ep,ev,re,r,rp)
fn retop(d: &mut Duel, a: &EffArgs) {
    // Duel.SendtoHand(e:GetHandler(),nil,REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    duel::send_to_hand(d, h, None, REASON_EFFECT, None);
}
