//! Earthbound Release
use super::prelude::*;

pub const ID: u32 = 84262395;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_EARTHBOUND], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(aux.FaceupFilter(Card.IsLevelAbove,6),1,nil)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &aux::faceup_filter(|d, c| card::is_level_above(d, c, 6)), 1, NO_EX)
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_EARTHBOUND) and c:IsLevel(10)
    card::is_set_card(d, c, &[SET_EARTHBOUND as u16]) && card::is_level(d, c, &[10])
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local dg=Duel.GetMatchingGroup(nil,tp,0,LOCATION_MZONE,nil)
    let dg = duel::get_matching_group(d, None, tp, 0, LOCATION_MZONE, NO_EX);
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.cfilter,1,false,aux.ReleaseCheckTarget,nil,dg) end
    if a.chk == Some(0) {
        let rc = |d: &mut Duel, sg: &CardSet, tp: u8, exg: &CardSet| aux::release_check_target(d, sg, tp, exg, &dg);
        return aux::check_release_group_cost(d, tp, Some(&cfilter), 1, 1, false, Some(&rc), NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.cfilter,1,1,false,aux.ReleaseCheckTarget,nil,dg)
    let rc = |d: &mut Duel, sg: &CardSet, tp: u8, exg: &CardSet| aux::release_check_target(d, sg, tp, exg, &dg);
    let g = aux::select_release_group_cost(d, tp, Some(&cfilter), 1, 1, false, Some(&rc), NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(nil,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, None, tp, 0, LOCATION_MZONE, NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // local dam=g:GetSum(Card.GetBaseAttack)
    let dam = group::get_sum(d, &g, &|d, c| card::get_base_attack(d, c) as i64);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,dam)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, dam as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local sg=Duel.GetMatchingGroup(nil,tp,0,LOCATION_MZONE,nil)
    let sg = duel::get_matching_group(d, None, tp, 0, LOCATION_MZONE, NO_EX);
    // if #sg>0 and Duel.Destroy(sg,REASON_EFFECT)>0 then
    if !sg.is_empty() && duel::destroy(d, sg.clone(), REASON_EFFECT, None, None) > 0 {
        // local dam=Duel.GetOperatedGroup():GetSum(Card.GetBaseAttack)
        let og = duel::get_operated_group(d);
        let dam = group::get_sum(d, &og, &|d, c| card::get_base_attack(d, c) as i64);
        // if dam==0 then return end
        if dam == 0 {
            return;
        }
        // Duel.Damage(1-tp,dam,REASON_EFFECT)
        duel::damage(d, 1 - tp, dam, REASON_EFFECT, false, None);
    }
}
