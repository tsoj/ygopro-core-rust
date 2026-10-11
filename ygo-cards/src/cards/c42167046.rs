//! Widespread Dud
use super::prelude::*;

pub const ID: u32 = 42167046;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EVENT_CHAIN_SOLVED)
    effect::set_code(d, e2, EVENT_CHAIN_SOLVED);
    // e2:SetLabelObject(e1)
    effect::set_label_object(d, e2, Value::Effect(e1));
    // e2:SetCondition(aux.PersistentTgCon)
    effect::set_condition(d, e2, Some(aux::persistent_tg_con()));
    // e2:SetOperation(s.tgop)
    effect::set_operation(d, e2, Some(operation(tgop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destroy1
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e3, EVENT_LEAVE_FIELD);
    // e3:SetCondition(s.descon1)
    effect::set_condition(d, e3, Some(condition(descon1)));
    // e3:SetOperation(s.desop1)
    effect::set_operation(d, e3, Some(operation(desop1)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Destroy2
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e4, EVENT_LEAVE_FIELD);
    // e4:SetCondition(s.descon2)
    effect::set_condition(d, e4, Some(condition(descon2)));
    // e4:SetOperation(s.desop2)
    effect::set_operation(d, e4, Some(operation(desop2)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsPosition(POS_FACEUP_ATTACK) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_position(d, chkc, POS_FACEUP_ATTACK);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsPosition,tp,LOCATION_MZONE,LOCATION_MZONE,2,nil,POS_FACEUP_ATTACK) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_position(d, c, POS_FACEUP_ATTACK)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 2, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUPATTACK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUPATTACK as u64);
    // Duel.SelectTarget(tp,Card.IsPosition,tp,LOCATION_MZONE,LOCATION_MZONE,2,2,nil,POS_FACEUP_ATTACK)
    let _ = duel::select_target(d, a.tp, Some(&|d, c| card::is_position(d, c, POS_FACEUP_ATTACK)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 2, 2, false, NO_EX);
    true
}
// function s.tgop(e,tp,eg,ep,ev,re,r,rp)
fn tgop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(re) then return end
    let re = a.re.nil();
    if !card::is_relate_to_effect(d, c, re) {
        return;
    }
    // local g=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS):Filter(Card.IsRelateToEffect,nil,re)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]);
    let targets = info[0].as_group().cloned().nil();
    let g = group::filter(d, &targets, &|d, x| card::is_relate_to_effect(d, x, re), NO_EX);
    // local tc=g:GetFirst()
    let _ = group::get_first(d, &g);
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // c:SetCardTarget(tc)
        card::set_card_target(d, c, tc);
    }
}
// function s.descon1(e,tp,eg,ep,ev,re,r,rp)
fn descon1(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsReason(REASON_DESTROY)
    let c = effect::get_handler(d, a.e).nil();
    card::is_reason(d, c, REASON_DESTROY)
}
// function s.desop1(e,tp,eg,ep,ev,re,r,rp)
fn desop1(d: &mut Duel, a: &EffArgs) {
    // local g=e:GetHandler():GetCardTarget():Filter(Card.IsLocation,nil,LOCATION_MZONE)
    let c = effect::get_handler(d, a.e).nil();
    let targets = card::get_card_target(d, c);
    let g = group::filter(d, &targets, &|d, x| card::is_location(d, x, LOCATION_MZONE), NO_EX);
    // Duel.Destroy(g,REASON_EFFECT)
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
// function s.descon2(e,tp,eg,ep,ev,re,r,rp)
fn descon2(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:GetCardTargetCount()==0 then return false end
    let g = card::get_card_target(d, c);
    if group::get_count(&g) == 0 {
        return false;
    }
    // local g=c:GetCardTarget()
    // local tc1=g:GetFirst()
    // local tc2=g:GetNext()
    let order = group::iter(d, &g);
    let tc1 = order.first().copied();
    let tc2 = order.get(1).copied();
    // return eg:IsContains(tc1) or (tc2 and eg:IsContains(tc2))
    let eg = a.eg.clone().nil();
    tc1.is_some_and(|x| group::is_contains(&eg, x)) || tc2.is_some_and(|x| group::is_contains(&eg, x))
}
// function s.desop2(e,tp,eg,ep,ev,re,r,rp)
fn desop2(d: &mut Duel, a: &EffArgs) {
    // Duel.Destroy(e:GetHandler(),REASON_EFFECT)
    let c = effect::get_handler(d, a.e).nil();
    duel::destroy(d, c, REASON_EFFECT, None, None);
}
