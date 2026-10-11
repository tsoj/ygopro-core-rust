//! Chaos Trap Hole
use super::prelude::*;

pub const ID: u32 = 11593137;

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
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(Cost.PayLP(2000))
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(2000))));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_FLIP_SUMMON)
    effect::set_code(d, e2, EVENT_FLIP_SUMMON);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    // e3:SetCode(EVENT_SPSUMMON)
    effect::set_code(d, e3, EVENT_SPSUMMON);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_DARK|ATTRIBUTE_LIGHT) and c:IsAbleToRemove()
    card::is_attribute(d, c, ATTRIBUTE_DARK | ATTRIBUTE_LIGHT) && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetCurrentChain(true)==0 and eg:IsExists(s.filter,1,nil)
    if duel::get_current_chain(d, true) != 0 {
        return false;
    }
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &filter, 1, NO_EX)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=eg:Filter(s.filter,nil)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &filter, NO_EX);
    let n = g.len() as u8;
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE_SUMMON,g,#g,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE_SUMMON, Some(g.clone()), n, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,#g,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), n, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=eg:Filter(s.filter,nil)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &filter, NO_EX);
    // Duel.NegateSummon(g)
    duel::negate_summon(d, g.clone());
    // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
    duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
