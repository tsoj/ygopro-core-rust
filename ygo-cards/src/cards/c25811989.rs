//! Infernoid Piaty
use super::prelude::*;

pub const ID: u32 = 25811989;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_INFERNOID],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Infernoid.RegisterSummonProcedure(c,2)
    aux::infernoid::register_summon_procedure(d, c, 2);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_HANDES)
    effect::set_category(d, e1, CATEGORY_HANDES);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e1, EVENT_BATTLE_DAMAGE);
    // e1:SetCondition(s.thcon)
    effect::set_condition(d, e1, Some(condition(thcon)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCost(s.rmcost)
    effect::set_cost(d, e2, Some(cost_target(rmcost)));
    // e2:SetTarget(s.rmtg)
    effect::set_target(d, e2, Some(cost_target(rmtg)));
    // e2:SetOperation(s.rmop)
    effect::set_operation(d, e2, Some(operation(rmop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==1-tp and Duel.GetAttacker()==e:GetHandler() and Duel.GetAttackTarget()~=nil
    a.ep == 1 - a.tp && duel::get_attacker(d) == Some(effect::get_handler(d, a.e).nil()) && duel::get_attack_target(d).is_some()
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND) > 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,0,0,1-tp,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, 1 - a.tp, 1);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetFieldGroup(ep,LOCATION_HAND,0)
    let g = duel::get_field_group(d, a.ep, LOCATION_HAND, 0);
    // local sg=g:RandomSelect(ep,1)
    let sg = group::random_select(d, &g, a.ep, 1);
    // Duel.SendtoGrave(sg,REASON_EFFECT)
    duel::send_to_grave(d, sg, REASON_EFFECT, None, None);
}
/// `s.rmfilter(c,e)`
fn rmfilter(d: &mut Duel, c: CardId, e: Option<EffectId>) -> bool {
    // return c:IsAbleToRemove() and aux.SpElimFilter(c) and (not e or c:IsCanBeEffectTarget(e))
    card::is_able_to_remove(d, c, None, None, None) && aux::sp_elim_filter(d, c, false, false) && (e.is_none() || card::is_can_be_effect_target(d, c, e))
}
/// `s.rmcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local dg=Duel.GetMatchingGroup(s.rmfilter,tp,0,LOCATION_MZONE|LOCATION_GRAVE,nil,e)
    let dg = duel::get_matching_group(d, Some(&|d, c| rmfilter(d, c, Some(a.e))), a.tp, 0, LOCATION_MZONE | LOCATION_GRAVE, NO_EX);
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,nil,1,false,aux.ReleaseCheckTarget,nil,dg) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&|_, _| true), 1, 1, false, Some(&|d, sg, tp, exg| aux::release_check_target(d, sg, tp, exg, &dg)), NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,nil,1,1,false,aux.ReleaseCheckTarget,nil,dg)
    let g = duel::select_release_group_cost(d, a.tp, Some(&|_, _| true), 1, 1, false, Some(&|d, sg, tp, exg| aux::release_check_target(d, sg, tp, exg, &dg)), NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE|LOCATION_GRAVE) and chkc:IsControler(1-tp) and s.rmfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE | LOCATION_GRAVE) && card::is_controler(d, chkc, 1 - a.tp) && rmfilter(d, chkc, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.rmfilter,tp,0,LOCATION_MZONE|LOCATION_GRAVE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| rmfilter(d, c, None)), a.tp, 0, LOCATION_MZONE | LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,s.rmfilter,tp,0,LOCATION_MZONE|LOCATION_GRAVE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| rmfilter(d, c, None)), a.tp, 0, LOCATION_MZONE | LOCATION_GRAVE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, 0, 0);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
