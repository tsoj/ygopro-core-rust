//! Cross Attack
use super::prelude::*;

pub const ID: u32 = 46961802;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter1(c,tp)`
fn filter1(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and Duel.IsExistingTarget(s.filter2,tp,LOCATION_MZONE,0,1,c,c:GetAttack())
    if !card::is_faceup(d, c) {
        return false;
    }
    let atk = card::get_attack(d, c);
    duel::is_existing_target(d, Some(&|d, x| filter2(d, x, atk)), tp, LOCATION_MZONE, 0, 1, c.into())
}
/// `s.filter2(c,atk)`
fn filter2(d: &mut Duel, c: CardId, atk: i32) -> bool {
    // return c:IsFaceup() and c:GetAttack()==atk
    card::is_faceup(d, c) && card::get_attack(d, c) == atk
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter1,tp,LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter1(d, c, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // local g1=Duel.SelectTarget(tp,s.filter1,tp,LOCATION_MZONE,0,1,1,nil,tp)
    let g1 = duel::select_target(d, a.tp, Some(&|d, c| filter1(d, c, a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // local g2=Duel.SelectTarget(tp,s.filter2,tp,LOCATION_MZONE,0,1,1,g1:GetFirst(),g1:GetFirst():GetAttack())
    let tc1 = group::get_first(d, &g1).nil();
    let atk = card::get_attack(d, tc1);
    let _g2 = duel::select_target(d, a.tp, Some(&|d, c| filter2(d, c, atk)), a.tp, LOCATION_MZONE, 0, 1, 1, false, tc1.into());
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]).first().and_then(|v| v.as_group().cloned()).nil();
    // local tc1=g:GetFirst()
    let tc1 = group::get_first(d, &g).nil();
    // local tc2=g:GetNext()
    let tc2 = group::iter(d, &g).get(1).copied();
    // if tc1:IsFacedown() or tc2:IsFacedown() or not tc1:IsRelateToEffect(e) or not tc2:IsRelateToEffect(e)
    //     or tc1:GetAttack()~=tc2:GetAttack() then return end
    if card::is_facedown(d, tc1)
        || card::is_facedown(d, tc2.nil())
        || !card::is_relate_to_effect(d, tc1, a.e)
        || !card::is_relate_to_effect(d, tc2.nil(), a.e)
        || card::get_attack(d, tc1) != card::get_attack(d, tc2.nil())
    {
        return;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // tc1:RegisterEffect(e1)
    card::register_effect(d, tc1, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // tc2:RegisterEffect(e2)
    card::register_effect(d, tc2.nil(), e2, false);
    // tc1:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
    card::register_flag_effect(d, tc1, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    // tc2:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
    card::register_flag_effect(d, tc2.nil(), ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    // local e3=Effect.CreateEffect(e:GetHandler())
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e3, EVENT_ATTACK_ANNOUNCE);
    // e3:SetOperation(s.atop)
    effect::set_operation(d, e3, Some(operation(atop)));
    // e3:SetLabelObject(tc2)
    effect::set_label_object(d, e3, Value::Card(tc2.nil()));
    // e3:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
    // tc1:RegisterEffect(e3)
    card::register_effect(d, tc1, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetLabelObject(tc1)
    effect::set_label_object(d, e4, Value::Card(tc1));
    // tc2:RegisterEffect(e4)
    card::register_effect(d, tc2.nil(), e4, false);
}
/// `s.atop(e,tp,eg,ep,ev,re,r,rp)`
fn atop(d: &mut Duel, a: &EffArgs) {
    // local oc=e:GetLabelObject()
    let oc = effect::get_label_object(d, a.e).as_card().nil();
    // if oc:GetFlagEffect(id)~=0 then
    if card::get_flag_effect(d, oc, ID) != 0 {
        // local e1=Effect.CreateEffect(e:GetOwner())
        let owner = effect::get_owner(d, a.e).nil();
        let e1 = effect::create_effect(d, owner);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CANNOT_ATTACK)
        effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // oc:RegisterEffect(e1,true)
        card::register_effect(d, oc, e1, true);
    }
}
