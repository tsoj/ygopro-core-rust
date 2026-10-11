//! Link Hole
use super::prelude::*;

pub const ID: u32 = 73275815;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(Card.IsLinkAbove,tp,LOCATION_MZONE,0,1,nil,4)
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_link_above(d, c, 4)), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ct=Duel.GetMatchingGroupCount(Card.IsLinkAbove,tp,LOCATION_MZONE,0,nil,3)
    let ct = duel::get_matching_group_count(d, Some(&|d, c| card::is_link_above(d, c, 3)), tp, LOCATION_MZONE, 0, NO_EX);
    // local dg=Duel.GetFieldGroup(tp,0,LOCATION_MZONE)
    let dg = duel::get_field_group(d, tp, 0, LOCATION_MZONE);
    // if chk==0 then return ct>0 and #dg>0 end
    if a.chk == Some(0) {
        return ct > 0 && !dg.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,dg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg), 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ct=Duel.GetMatchingGroupCount(Card.IsLinkAbove,tp,LOCATION_MZONE,0,nil,3)
    let ct = duel::get_matching_group_count(d, Some(&|d, c| card::is_link_above(d, c, 3)), tp, LOCATION_MZONE, 0, NO_EX);
    // if ct<1 then return end
    if ct < 1 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectMatchingCard(tp,nil,tp,0,LOCATION_MZONE,1,ct,nil)
    let g = duel::select_matching_card(d, tp, None, tp, 0, LOCATION_MZONE, 1, ct as u16, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
