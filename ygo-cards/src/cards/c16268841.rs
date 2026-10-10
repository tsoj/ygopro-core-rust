//! Zolga
use super::prelude::*;

pub const ID: u32 = 16268841;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // recover
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_RECOVER)
    effect::set_category(d, e1, CATEGORY_RECOVER);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EVENT_RELEASE)
    effect::set_code(d, e1, EVENT_RELEASE);
    // e1:SetCondition(s.reccon)
    effect::set_condition(d, e1, Some(condition(reccon)));
    // e1:SetTarget(s.rectg)
    effect::set_target(d, e1, Some(cost_target(rectg)));
    // e1:SetOperation(s.recop)
    effect::set_operation(d, e1, Some(operation(recop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.reccon(e,tp,eg,ep,ev,re,r,rp)`
fn reccon(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(e:GetHandler():GetPreviousControler())
    let h = effect::get_handler(d, a.e).nil();
    let prev = card::get_previous_controler(d, h);
    effect::set_label(d, a.e, &[prev as i64]);
    // return e:GetHandler():IsReason(REASON_SUMMON)
    card::is_reason(d, h, REASON_SUMMON)
}
/// `s.rectg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rectg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckEvent(EVENT_SUMMON_SUCCESS) or Duel.CheckEvent(EVENT_MSET) end
    if a.chk == Some(0) {
        return duel::check_event(d, EVENT_SUMMON_SUCCESS).is_some() || duel::check_event(d, EVENT_MSET).is_some();
    }
    // Duel.SetTargetPlayer(e:GetLabel())
    let label = effect::get_label(d, a.e) as u8;
    duel::set_target_player(d, label);
    // Duel.SetTargetParam(2000)
    duel::set_target_param(d, 2000);
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,e:GetLabel(),2000)
    let label = effect::get_label(d, a.e) as u8;
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, label, 2000);
    true
}
/// `s.recop(e,tp,eg,ep,ev,re,r,rp)`
fn recop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    let amount = info[1].int();
    // Duel.Recover(p,d,REASON_EFFECT)
    duel::recover(d, p, amount, REASON_EFFECT, false, None);
}
