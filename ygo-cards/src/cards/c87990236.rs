//! Darklord Enchantment
use super::prelude::*;

pub const ID: u32 = 87990236;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DARKLORD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Take control of 1 monster your opponent controls
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e1, CATEGORY_CONTROL);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.costfilter(c,tp)
fn costfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_DARKLORD) and c:IsMonster() and (c:IsLocation(LOCATION_HAND) or c:IsFaceup())
    //     and c:IsAbleToGraveAsCost() and Duel.GetMZoneCount(tp,c,tp,LOCATION_REASON_CONTROL)>0
    card::is_set_card(d, c, &[SET_DARKLORD as u16])
        && card::is_monster(d, c)
        && (card::is_location(d, c, LOCATION_HAND) || card::is_faceup(d, c))
        && card::is_able_to_grave_as_cost(d, c)
        && duel::get_m_zone_count(d, tp, c.into(), Some(tp), Some(LOCATION_REASON_CONTROL), None).0 > 0
}
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // e:SetLabel(100)
    effect::set_label(d, a.e, &[100]);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.costfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| costfilter(d, c, tp)), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.costfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,1,nil,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| costfilter(d, c, tp)), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
// function s.filter(c,ignzn)
fn filter(d: &mut Duel, c: CardId, ignzn: bool) -> bool {
    // return c:IsControlerCanBeChanged(ignzn) and c:IsFaceup()
    card::is_controler_can_be_changed(d, c, ignzn, None) && card::is_faceup(d, c)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local ignzn=e:GetLabel()==100
        let ignzn = effect::get_label(d, a.e) == 100;
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return Duel.IsExistingMatchingCard(s.filter,tp,0,LOCATION_MZONE,1,nil,ignzn) end
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, ignzn)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, None, 1, 0, 0);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONTROL as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,0,LOCATION_MZONE,1,1,nil,false)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| filter(d, c, false)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    // if tc then
    if let Some(tc) = group::get_first(d, &g) {
        // Duel.GetControl(tc,tp,PHASE_END,1)
        duel::get_control(d, tc, tp, PHASE_END as u16, 1, None, None);
    }
}
