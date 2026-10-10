//! Royal Oppression
use super::prelude::*;

pub const ID: u32 = 93016201;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Activate(timing)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e2, EFFECT_TYPE_ACTIVATE);
    // e2:SetCode(EVENT_SPSUMMON)
    effect::set_code(d, e2, EVENT_SPSUMMON);
    // e2:SetCondition(s.condition1)
    effect::set_condition(d, e2, Some(condition(condition1)));
    // e2:SetCost(Cost.PayLP(800))
    effect::set_cost(d, e2, Some(cost::pay_lp(cost::PayLp::Amount(800))));
    // e2:SetTarget(s.target2)
    effect::set_target(d, e2, Some(cost_target(target2)));
    // e2:SetOperation(s.activate2)
    effect::set_operation(d, e2, Some(operation(activate2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // instant
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetProperty(EFFECT_FLAG_BOTH_SIDE)
    effect::set_property(d, e3, EFFECT_FLAG_BOTH_SIDE, 0);
    // e3:SetCode(EVENT_SPSUMMON)
    effect::set_code(d, e3, EVENT_SPSUMMON);
    // e3:SetCondition(s.condition2)
    effect::set_condition(d, e3, Some(condition(condition2)));
    // e3:SetCost(Cost.PayLP(800))
    effect::set_cost(d, e3, Some(cost::pay_lp(cost::PayLp::Amount(800))));
    // e3:SetTarget(s.target2)
    effect::set_target(d, e3, Some(cost_target(target2)));
    // e3:SetOperation(s.activate2)
    effect::set_operation(d, e3, Some(operation(activate2)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // instant(chain)
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetProperty(EFFECT_FLAG_BOTH_SIDE)
    effect::set_property(d, e4, EFFECT_FLAG_BOTH_SIDE, 0);
    // e4:SetCode(EVENT_CHAINING)
    effect::set_code(d, e4, EVENT_CHAINING);
    // e4:SetCondition(s.condition3)
    effect::set_condition(d, e4, Some(condition(condition3)));
    // e4:SetCost(Cost.PayLP(800))
    effect::set_cost(d, e4, Some(cost::pay_lp(cost::PayLp::Amount(800))));
    // e4:SetTarget(s.target3)
    effect::set_target(d, e4, Some(cost_target(target3)));
    // e4:SetOperation(s.activate3)
    effect::set_operation(d, e4, Some(operation(activate3)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.condition1(...)`: `not Duel.IsDuelType(DUEL_USE_TRAPS_IN_NEW_CHAIN) and s.condition2(...)`
fn condition1(d: &mut Duel, a: &EffArgs) -> bool {
    !duel::is_duel_type(d, DUEL_USE_TRAPS_IN_NEW_CHAIN) && condition2(d, a)
}
/// `s.condition2(e,tp,eg,ep,ev,re,r,rp)`
fn condition2(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.GetCurrentChain()==0
    duel::get_current_chain(d, false) == 0
}
/// `s.target2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE_SUMMON,eg,#eg,0,0)
    let n = a.eg.clone().nil().len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DISABLE_SUMMON, a.eg.clone(), n, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,#eg,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), n, 0, 0);
    true
}
/// `s.activate2(e,tp,eg,ep,ev,re,r,rp)`
fn activate2(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // Duel.NegateSummon(eg)
    duel::negate_summon(d, a.eg());
    // Duel.Destroy(eg,REASON_EFFECT)
    duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
}
/// `s.condition3(e,tp,eg,ep,ev,re,r,rp)`
fn condition3(d: &mut Duel, a: &EffArgs) -> bool {
    // return re:IsHasCategory(CATEGORY_SPECIAL_SUMMON) and Duel.IsChainDisablable(ev)
    let re = a.re.nil();
    effect::is_has_category(d, re, CATEGORY_SPECIAL_SUMMON) && duel::is_chain_disablable(d, a.ev as u8)
}
/// `s.target3(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target3(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, 0, 0);
    // if re:GetHandler():IsDestructable() and re:GetHandler():IsRelateToEffect(re) then
    let re = a.re.nil();
    let rh = effect::get_handler(d, re).nil();
    if card::is_destructable(d, rh, None) && card::is_relate_to_effect(d, rh, re) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
/// `s.activate3(e,tp,eg,ep,ev,re,r,rp)`
fn activate3(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // if Duel.NegateEffect(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_effect(d, a.ev as u8) {
        let re = a.re.nil();
        let rh = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rh, re) {
            // Duel.Destroy(eg,REASON_EFFECT)
            duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
        }
    }
}
