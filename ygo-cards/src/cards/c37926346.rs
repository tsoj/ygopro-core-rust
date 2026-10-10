//! Steelswarm Roach
use super::prelude::*;

pub const ID: u32 = 37926346;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Xyz summon
    // Xyz.AddProcedure(c,nil,4,2)
    xyz::add_procedure(d, c, None, Some(4), 2, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // disable spsummon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DISABLE_SUMMON+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DISABLE_SUMMON + CATEGORY_DESTROY);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EVENT_SPSUMMON)
    effect::set_code(d, e1, EVENT_SPSUMMON);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelAbove(5)
    card::is_level_above(d, c, 5)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetCurrentChain()==0 and eg:IsExists(s.filter,1,nil)
    if duel::get_current_chain(d, false) != 0 {
        return false;
    }
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| filter(d, c), 1, NO_EX)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=eg:Filter(s.filter,nil)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| filter(d, c), NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE_SUMMON,g,#g,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE_SUMMON, Some(g.clone()), g.len() as u8, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), g.len() as u8, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp,chk)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local g=eg:Filter(s.filter,nil)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| filter(d, c), NO_EX);
    // Duel.NegateSummon(g)
    duel::negate_summon(d, g.clone());
    // Duel.Destroy(g,REASON_EFFECT)
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
