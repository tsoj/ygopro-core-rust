//! Darklight
use super::prelude::*;

pub const ID: u32 = 29876529;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // return true
    true
}
/// `s.costfilter(c)`
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_DARK) and c:IsAttackAbove(1500)
    //     and Duel.IsExistingMatchingCard(s.dfilter,0,LOCATION_MZONE,LOCATION_MZONE,1,c)
    card::is_attribute(d, c, ATTRIBUTE_DARK) && card::is_attack_above(d, c, 1500) && duel::is_existing_matching_card(d, Some(&dfilter), 0, LOCATION_MZONE, LOCATION_MZONE, 1, c.into())
}
/// `s.dfilter(c)`
fn dfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsStatus(STATUS_SPSUMMON_TURN)
    card::is_status(d, c, STATUS_SPSUMMON_TURN)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()~=0 then
        if effect::get_label(d, e) != 0 {
            // e:SetLabel(0)
            effect::set_label(d, e, &[0]);
            // return Duel.CheckReleaseGroupCost(tp,s.costfilter,1,false,nil,nil)
            return duel::check_release_group_cost(d, tp, Some(&costfilter), 1, 1, false, None, NO_EX);
        } else {
            // return Duel.IsExistingMatchingCard(s.dfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
            return duel::is_existing_matching_card(d, Some(&dfilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
        }
    }
    // if e:GetLabel()==1 then
    if effect::get_label(d, e) == 1 {
        // e:SetLabel(0)
        effect::set_label(d, e, &[0]);
        // local rg=Duel.SelectReleaseGroupCost(tp,s.costfilter,1,1,false,nil,nil)
        let rg = duel::select_release_group_cost(d, tp, Some(&costfilter), 1, 1, false, None, NO_EX);
        // Duel.Release(rg,REASON_COST)
        duel::release(d, rg, REASON_COST, None);
    }
    // local g=Duel.GetMatchingGroup(s.dfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&dfilter), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.dfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&dfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.Destroy(g,REASON_EFFECT)
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
