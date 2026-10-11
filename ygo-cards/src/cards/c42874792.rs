//! Wind-Up Rabbit
use super::prelude::*;

pub const ID: u32 = 42874792;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_WIND_UP], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // remove
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetProperty(EFFECT_FLAG_NO_TURN_RESET+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_NO_TURN_RESET + EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_WIND_UP) and c:IsAbleToRemove()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_WIND_UP as u16]) && card::is_able_to_remove(d, c, None, None, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), 1, 0, 0);
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.Remove(tc,tc:GetPosition(),REASON_EFFECT|REASON_TEMPORARY)~=0 then
    if card::is_relate_to_effect(d, tc, a.e) {
        let pos = card::get_position(d, tc) as u8;
        if duel::remove(d, tc, pos, REASON_EFFECT | REASON_TEMPORARY, None, None) != 0 {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            // e1:SetCode(EVENT_PHASE|PHASE_STANDBY)
            effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
            // e1:SetRange(LOCATION_REMOVED)
            effect::set_range(d, e1, LOCATION_REMOVED);
            // e1:SetCountLimit(1)
            effect::set_count_limit(d, e1, 1, 0, 0, 0);
            // if Duel.IsTurnPlayer(tp) then
            if duel::is_turn_player(d, tp) {
                let turn = duel::get_turn_count(d, None) as i64;
                // if Duel.IsPhase(PHASE_DRAW) then
                if duel::is_phase(d, PHASE_DRAW) {
                    // e1:SetLabel(Duel.GetTurnCount())
                    effect::set_label(d, e1, &[turn]);
                } else {
                    // e1:SetLabel(Duel.GetTurnCount()+2)
                    effect::set_label(d, e1, &[turn + 2]);
                }
            } else {
                // e1:SetLabel(Duel.GetTurnCount()+1)
                let turn = duel::get_turn_count(d, None) as i64;
                effect::set_label(d, e1, &[turn + 1]);
            }
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // e1:SetCondition(s.retcon)
            effect::set_condition(d, e1, Some(condition(retcon)));
            // e1:SetOperation(s.retop)
            effect::set_operation(d, e1, Some(operation(retop)));
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
// function s.retcon(e,tp,eg,ep,ev,re,r,rp)
fn retcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetTurnCount()==e:GetLabel()
    duel::get_turn_count(d, None) as i64 == effect::get_label(d, a.e)
}
// function s.retop(e,tp,eg,ep,ev,re,r,rp)
fn retop(d: &mut Duel, a: &EffArgs) {
    // Duel.ReturnToField(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    duel::return_to_field(d, h, None, None);
    // e:Reset()
    effect::reset(d, a.e);
}
