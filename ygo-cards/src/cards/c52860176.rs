//! Possessed Dark Soul
use super::prelude::*;

pub const ID: u32 = 52860176;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You can Tribute this face-up card; take control of all face-up Level 3 or lower monsters your opponent currently controls
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e1, CATEGORY_CONTROL);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e1, Some(cost::self_tribute()));
    // e1:SetTarget(s.controltg)
    effect::set_target(d, e1, Some(cost_target(controltg)));
    // e1:SetOperation(s.controlop)
    effect::set_operation(d, e1, Some(operation(controlop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.controlfilter(c)`
fn controlfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelBelow(3) and c:IsFaceup() and c:IsControlerCanBeChanged(true)
    card::is_level_below(d, c, 3) && card::is_faceup(d, c) && card::is_controler_can_be_changed(d, c, true, None)
}
/// `s.controltg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn controltg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // local ctrl_count=Duel.GetMatchingGroupCount(s.controlfilter,tp,0,LOCATION_MZONE,nil)
        let ctrl_count = duel::get_matching_group_count(d, Some(&controlfilter), a.tp, 0, LOCATION_MZONE, NO_EX);
        // return ctrl_count>0 and Duel.GetMZoneCount(tp,e:GetHandler(),tp,LOCATION_REASON_CONTROL)>=ctrl_count
        let h = effect::get_handler(d, a.e).nil();
        return ctrl_count > 0 && duel::get_m_zone_count(d, a.tp, h.into(), Some(a.tp), Some(LOCATION_REASON_CONTROL), None).0 >= ctrl_count as i32;
    }
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,nil,1,1-tp,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, None, 1, 1 - a.tp, LOCATION_MZONE as i32);
    true
}
/// `s.controlop(e,tp,eg,ep,ev,re,r,rp)`
fn controlop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.controlfilter,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&controlfilter), a.tp, 0, LOCATION_MZONE, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.GetControl(g,tp)
        duel::get_control(d, g, a.tp, 0, 0, None, None);
    }
}
