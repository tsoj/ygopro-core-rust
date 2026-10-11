//! Oni-Gami Combo
use super::prelude::*;

pub const ID: u32 = 90470931;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // multi attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
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
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsAbleToEnterBP()
    duel::is_able_to_enter_bp(d)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // return true
    true
}
/// `s.filter(c,cst)`
fn filter(d: &mut Duel, c: CardId, cst: bool) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_XYZ) and not c:IsHasEffect(EFFECT_EXTRA_ATTACK) and (not cst or c:GetOverlayCount()~=0)
    card::is_faceup(d, c)
        && card::is_type(d, c, TYPE_XYZ)
        && card::is_has_effect(d, c, EFFECT_EXTRA_ATTACK, None).is_empty()
        && (!cst || card::get_overlay_count(d, c) != 0)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local chkcost=e:GetLabel()==1 and true or false
    let chkcost = effect::get_label(d, e) == 1;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc,chkcost) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && filter(d, chkc, chkcost);
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // e:SetLabel(0)
        effect::set_label(d, e, &[0]);
        // return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil,chkcost)
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, chkcost)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // e:SetLabel(0)
    effect::set_label(d, e, &[0]);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // local tc=Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil,chkcost):GetFirst()
    let sg = duel::select_target(d, tp, Some(&|d, c| filter(d, c, chkcost)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sg).nil();
    // if chkcost then
    if chkcost {
        // tc:RemoveOverlayCard(tp,tc:GetOverlayCount(),tc:GetOverlayCount(),REASON_COST)
        let n = card::get_overlay_count(d, tc) as u16;
        card::remove_overlay_card(d, tc, tp, n, n, REASON_COST);
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_EXTRA_ATTACK)
        effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
