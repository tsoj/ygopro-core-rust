//! Majespecter Tempest
use super::prelude::*;

pub const ID: u32 = 2572890;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate(summon)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DISABLE_SUMMON+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DISABLE_SUMMON | CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_SPSUMMON)
    effect::set_code(d, e1, EVENT_SPSUMMON);
    // e1:SetCondition(s.condition1)
    effect::set_condition(d, e1, Some(condition(condition1)));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target1)
    effect::set_target(d, e1, Some(cost_target(target1)));
    // e1:SetOperation(s.activate1)
    effect::set_operation(d, e1, Some(operation(activate1)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Activate(effect)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_NEGATE | CATEGORY_DESTROY);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP | EFFECT_FLAG_DAMAGE_CAL, 0);
    // e2:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e2, EFFECT_TYPE_ACTIVATE);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetCondition(s.condition2)
    effect::set_condition(d, e2, Some(condition(condition2)));
    // e2:SetCost(s.cost)
    effect::set_cost(d, e2, Some(cost_target(cost_fn)));
    // e2:SetTarget(s.target2)
    effect::set_target(d, e2, Some(cost_target(target2)));
    // e2:SetOperation(s.activate2)
    effect::set_operation(d, e2, Some(operation(activate2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_SPELLCASTER) and c:IsAttribute(ATTRIBUTE_WIND) and not c:IsStatus(STATUS_BATTLE_DESTROYED)
    card::is_race(d, c, RACE_SPELLCASTER as u64) && card::is_attribute(d, c, ATTRIBUTE_WIND) && !card::is_status(d, c, STATUS_BATTLE_DESTROYED)
}
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.cfilter,1,false,nil,nil) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&cfilter), 1, 1, false, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.cfilter,1,1,false,nil,nil)
    let g = duel::select_release_group_cost(d, a.tp, Some(&cfilter), 1, 1, false, None, NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
// function s.condition1(e,tp,eg,ep,ev,re,r,rp)
fn condition1(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.GetCurrentChain(true)==0
    duel::get_current_chain(d, true) == 0
}
// function s.target1(e,tp,eg,ep,ev,re,r,rp,chk)
fn target1(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE_SUMMON,eg,#eg,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE_SUMMON, a.eg.clone(), group::get_count(&a.eg()) as u8, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,#eg,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), group::get_count(&a.eg()) as u8, 0, 0);
    true
}
// function s.activate1(e,tp,eg,ep,ev,re,r,rp)
fn activate1(d: &mut Duel, a: &EffArgs) {
    // Duel.NegateSummon(eg)
    duel::negate_summon(d, a.eg());
    // Duel.Destroy(eg,REASON_EFFECT)
    duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
}
// function s.condition2(e,tp,eg,ep,ev,re,r,rp)
fn condition2(d: &mut Duel, a: &EffArgs) -> bool {
    // return re:IsMonsterEffect() and Duel.IsChainNegatable(ev)
    effect::is_monster_effect(d, a.re.nil()) && duel::is_chain_negatable(d, a.ev as u8)
}
// function s.target2(e,tp,eg,ep,ev,re,r,rp,chk)
fn target2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // if re:GetHandler():IsDestructable() and re:GetHandler():IsRelateToEffect(re) then
    let rh = effect::get_handler(d, a.re.nil()).nil();
    if card::is_destructable(d, rh, None) && card::is_relate_to_effect(d, rh, a.re.nil()) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
// function s.activate2(e,tp,eg,ep,ev,re,r,rp)
fn activate2(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) && card::is_relate_to_effect(d, effect::get_handler(d, a.re.nil()).nil(), a.re.nil()) {
        // Duel.Destroy(eg,REASON_EFFECT)
        duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
    }
}
