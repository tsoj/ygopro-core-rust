//! Synchro Gift
use super::prelude::*;

pub const ID: u32 = 52128900;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
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
/// `s.filter1(c)`
fn filter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_SYNCHRO)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_SYNCHRO)
}
/// `s.filter2(c)`
fn filter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and not c:IsType(TYPE_SYNCHRO)
    card::is_faceup(d, c) && !card::is_type(d, c, TYPE_SYNCHRO)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter1,tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingTarget(s.filter2,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter1), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&filter2), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,0))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 0));
    // local g1=Duel.SelectTarget(tp,s.filter1,tp,LOCATION_MZONE,0,1,1,nil)
    let g1 = duel::select_target(d, a.tp, Some(&filter1), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 1));
    // local g2=Duel.SelectTarget(tp,s.filter2,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, a.tp, Some(&filter2), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // e:SetLabelObject(g1:GetFirst())
    let tc1 = group::get_first(d, &g1.nil());
    let label = match tc1 {
        Some(tc) => Value::Card(tc),
        None => Value::Nil,
    };
    effect::set_label_object(d, a.e, label);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc1=e:GetLabelObject()
    let tc1 = effect::get_label_object(d, a.e).as_card();
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]);
    let g = info[0].as_group().cloned().nil();
    // local tc2=g:GetFirst()
    let order = group::iter(d, &g);
    let mut tc2 = order.first().copied();
    // if tc1==tc2 then tc2=g:GetNext() end
    if tc1 == tc2 {
        tc2 = order.get(1).copied();
    }
    // if tc1:IsRelateToEffect(e) and tc1:IsFaceup() and tc2:IsRelateToEffect(e) and tc2:IsFaceup() then
    if card::is_relate_to_effect(d, tc1.nil(), a.e)
        && card::is_faceup(d, tc1.nil())
        && card::is_relate_to_effect(d, tc2.nil(), a.e)
        && card::is_faceup(d, tc2.nil())
    {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // e1:SetValue(tc1:GetBaseAttack())
        let base = card::get_base_attack(d, tc1.nil());
        effect::set_value(d, e1, base as i64);
        // tc2:RegisterEffect(e1)
        card::register_effect(d, tc2.nil(), e1, false);
        // local e2=Effect.CreateEffect(e:GetHandler())
        let e2 = effect::create_effect(d, h);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e2, EFFECT_SET_ATTACK_FINAL);
        // e2:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        // e2:SetValue(0)
        effect::set_value(d, e2, 0);
        // tc1:RegisterEffect(e2)
        card::register_effect(d, tc1.nil(), e2, false);
    }
}
