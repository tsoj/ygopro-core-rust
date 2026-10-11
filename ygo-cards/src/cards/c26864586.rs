//! Resonance Device
use super::prelude::*;

pub const ID: u32 = 26864586;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// s.filter1(c,tp)
fn filter1(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // local lv1=c:GetLevel()
    let lv1 = card::get_level(d, c);
    // return lv1>0 and c:IsFaceup() and Duel.IsExistingTarget(s.filter2,tp,LOCATION_MZONE,0,1,c,c:GetRace(),c:GetAttribute(),lv1)
    if lv1 == 0 || !card::is_faceup(d, c) {
        return false;
    }
    let race = card::get_race(d, c);
    let attr = card::get_attribute(d, c);
    duel::is_existing_target(d, Some(&|d, x| filter2(d, x, race, attr, lv1)), tp, LOCATION_MZONE, 0, 1, c.into())
}
// s.filter2(c,rc,at,lv1)
fn filter2(d: &mut Duel, c: CardId, rc: u64, at: u32, lv1: u32) -> bool {
    // local lv2=c:GetLevel()
    let lv2 = card::get_level(d, c);
    // return lv2>0 and lv2~=lv1 and c:IsFaceup() and c:IsRace(rc) and c:IsAttribute(at)
    lv2 > 0 && lv2 != lv1 && card::is_faceup(d, c) && card::is_race(d, c, rc) && card::is_attribute(d, c, at)
}
// s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
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
    // local tc1=g1:GetFirst()
    let tc1 = group::get_first(d, &g1);
    // e:SetLabelObject(tc1)
    effect::set_label_object(d, a.e, Value::Card(tc1.nil()));
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,0))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 0));
    // local g2=Duel.SelectTarget(tp,s.filter2,tp,LOCATION_MZONE,0,1,1,tc1,tc1:GetRace(),tc1:GetAttribute(),tc1:GetLevel())
    let tc1c = tc1.nil();
    let race = card::get_race(d, tc1c);
    let attr = card::get_attribute(d, tc1c);
    let lv = card::get_level(d, tc1c);
    let _g2 = duel::select_target(d, a.tp, Some(&|d, c| filter2(d, c, race, attr, lv)), a.tp, LOCATION_MZONE, 0, 1, 1, false, tc1c.into());
    true
}
// s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]);
    let g = info[0].as_group().cloned().nil();
    // local tc1=e:GetLabelObject()
    let tc1 = effect::get_label_object(d, a.e).as_card();
    // local tc2=g:GetFirst()
    let order = group::iter(d, &g);
    let mut tc2 = order.first().copied();
    // if tc1==tc2 then tc2=g:GetNext() end
    if tc1 == tc2 {
        tc2 = order.get(1).copied();
    }
    // local lv1=tc1:GetLevel()
    let lv1 = card::get_level(d, tc1.nil());
    // local lv2=tc2:GetLevel()
    let lv2 = card::get_level(d, tc2.nil());
    // if lv1==lv2 then return end
    if lv1 == lv2 {
        return;
    }
    // if tc1:IsFaceup() and tc1:IsRelateToEffect(e) and tc2:IsFaceup() and tc2:IsRelateToEffect(e) then
    let (tc1, tc2) = (tc1.nil(), tc2.nil());
    if card::is_faceup(d, tc1) && card::is_relate_to_effect(d, tc1, a.e) && card::is_faceup(d, tc2) && card::is_relate_to_effect(d, tc2, a.e) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_LEVEL)
        effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
        // e1:SetValue(lv1)
        effect::set_value(d, e1, lv1 as i64);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc2:RegisterEffect(e1)
        card::register_effect(d, tc2, e1, false);
    }
}
