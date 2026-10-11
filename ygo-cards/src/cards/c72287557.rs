//! Chthonian Polymer
//! ヘル・ポリマー
use super::prelude::*;

pub const ID: u32 = 72287557;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e1, CATEGORY_CONTROL);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=eg:GetFirst()
    let eg = a.eg();
    let tc = group::get_first(d, &eg);
    // return #eg==1 and tc:IsControler(1-tp) and tc:IsFusionSummoned()
    eg.len() == 1 && {
        let tc = tc.nil();
        card::is_controler(d, tc, 1 - a.tp) && card::is_fusion_summoned(d, tc)
    }
}
/// `s.cfilter(c,tp,eg)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8, eg: &CardSet) -> bool {
    // return Duel.GetMZoneCount(tp,c)>0 and not eg:IsContains(c)
    duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0 && !group::is_contains(eg, c)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg();
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.cfilter,1,false,nil,nil,tp,eg) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&|d, c| cfilter(d, c, a.tp, &eg)), 1, 1, false, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.cfilter,1,1,false,nil,nil,tp,eg)
    let g = duel::select_release_group_cost(d, a.tp, Some(&|d, c| cfilter(d, c, a.tp, &eg)), 1, 1, false, None, NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    let eg = a.eg();
    // if chk==0 then return eg:GetFirst():IsCanBeEffectTarget(e) and eg:GetFirst():IsAbleToChangeControler() end
    if a.chk == Some(0) {
        let tc = group::get_first(d, &eg).nil();
        return card::is_can_be_effect_target(d, tc, Some(a.e)) && card::is_able_to_change_controler(d, tc);
    }
    // Duel.SetTargetCard(eg)
    duel::set_target_card(d, eg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(eg), 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.GetControl(tc,tp)
        duel::get_control(d, tc, a.tp, 0, 0, None, None);
    }
}
