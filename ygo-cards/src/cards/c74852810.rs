//! Overlay Capture
use super::prelude::*;

pub const ID: u32 = 74852810;

pub fn initial_effect(d: &mut Duel, c: CardId) {
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
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter1(c)`
fn filter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_XYZ) and c:GetOverlayCount()>0
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ) && card::get_overlay_count(d, c) > 0
}
/// `s.filter2(c)`
fn filter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_XYZ)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ)
}
/// `s.target(e,tp,...,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return e:IsHasType(EFFECT_TYPE_ACTIVATE) and Duel.IsExistingTarget(s.filter1,tp,0,LOCATION_MZONE,1,nil)
    //     and Duel.IsExistingTarget(s.filter2,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE)
            && duel::is_existing_target(d, Some(&filter1), tp, 0, LOCATION_MZONE, 1, NO_EX)
            && duel::is_existing_target(d, Some(&filter2), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local g1=Duel.SelectTarget(tp,s.filter1,tp,0,LOCATION_MZONE,1,1,nil)
    let g1 = duel::select_target(d, tp, Some(&filter1), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // e:SetLabelObject(g1:GetFirst())
    let first = group::get_first(d, &g1).nil();
    effect::set_label_object(d, e, Value::Card(first));
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.filter2,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, tp, Some(&filter2), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.operation(e,tp,...)`
fn op(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc1=e:GetLabelObject()
    let tc1 = effect::get_label_object(d, e).as_card().nil();
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    // local tc2=g:GetFirst()
    let mut tc2 = group::get_first(d, &g);
    // if tc1==tc2 then tc2=g:GetNext() end
    if Some(tc1) == tc2 {
        tc2 = group::iter(d, &g).get(1).copied();
    }
    // if tc1:IsFacedown() or not tc1:IsRelateToEffect(e) then return end
    if card::is_facedown(d, tc1) || !card::is_relate_to_effect(d, tc1, e) {
        return;
    }
    // local og=tc1:GetOverlayGroup()
    let og = card::get_overlay_group(d, tc1);
    // if #og==0 then return end
    if og.is_empty() {
        return;
    }
    // if Duel.SendtoGrave(og,REASON_EFFECT)~=0 and tc2:IsFaceup() and tc2:IsRelateToEffect(e) and c:IsRelateToEffect(e) then
    if duel::send_to_grave(d, og, REASON_EFFECT, None, None) != 0
        && card::is_faceup(d, tc2.nil())
        && card::is_relate_to_effect(d, tc2.nil(), e)
        && card::is_relate_to_effect(d, c, e)
    {
        // c:CancelToGrave()
        card::cancel_to_grave(d, c, true);
        // Duel.Overlay(tc2,c)
        duel::overlay(d, tc2.nil(), c, false);
    }
}
