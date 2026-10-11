//! D.D. Seeker
use super::prelude::*;

pub const ID: u32 = 89015998;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // remove
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.rmtg)
    effect::set_target(d, e1, Some(cost_target(rmtg)));
    // e1:SetOperation(s.rmop)
    effect::set_operation(d, e1, Some(operation(rmop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.rmfilter(c)
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAbleToRemove()
    card::is_faceup(d, c) && card::is_able_to_remove(d, c, None, None, None)
}
// function s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.rmfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && rmfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.rmfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&rmfilter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,s.rmfilter,tp,LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&rmfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, 0, 0);
    true
}
// function s.rmop(e,tp,eg,ep,ev,re,r,rp)
fn rmop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.Remove(tc,0,REASON_EFFECT|REASON_TEMPORARY)~=0 then
    if card::is_relate_to_effect(d, tc, e) && duel::remove(d, tc, 0, REASON_EFFECT | REASON_TEMPORARY, None, None) != 0 {
        // local ct=2
        let ct: u16 = 2;
        // tc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,ct)
        card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, 0, ct, 0, 0);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
        // e1:SetReset(RESET_PHASE|PHASE_END,ct)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, ct);
        // e1:SetLabelObject(tc)
        effect::set_label_object(d, e1, Value::Card(tc));
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // e1:SetCondition(s.retcon)
        effect::set_condition(d, e1, Some(condition(retcon)));
        // e1:SetOperation(s.retop)
        effect::set_operation(d, e1, Some(operation(retop)));
        // e1:SetLabel(Duel.GetTurnCount())
        let turn = duel::get_turn_count(d, None);
        effect::set_label(d, e1, &[turn as i64]);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
    }
}
// function s.retcon(e,tp,eg,ep,ev,re,r,rp)
fn retcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetTurnCount()>e:GetLabel() and e:GetLabelObject():GetFlagEffect(id)~=0
    let label = effect::get_label(d, a.e);
    duel::get_turn_count(d, None) as i64 > label
        && card::get_flag_effect(d, effect::get_label_object(d, a.e).as_card().nil(), ID) != 0
}
// function s.retop(e,tp,eg,ep,ev,re,r,rp)
fn retop(d: &mut Duel, a: &EffArgs) {
    // Duel.ReturnToField(e:GetLabelObject())
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    duel::return_to_field(d, tc, None, None);
}
