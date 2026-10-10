//! Tailor of the Fickle
use super::prelude::*;

pub const ID: u32 = 43641473;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetHintTiming(0,TIMING_EQUIP)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_EQUIP));
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.tcfilter(tc,ec)`
fn tcfilter(d: &mut Duel, tc: CardId, ec: CardId) -> bool {
    // return tc:IsFaceup() and ec:CheckEquipTarget(tc)
    card::is_faceup(d, tc) && card::check_equip_target(d, ec, tc)
}
/// `s.ecfilter(c)`
fn ecfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_EQUIP) and (not c:IsOriginalType(TYPE_MONSTER) or c:IsOriginalType(TYPE_UNION))
    //     and c:GetEquipTarget()~=nil and Duel.IsExistingTarget(s.tcfilter,0,LOCATION_MZONE,LOCATION_MZONE,1,c:GetEquipTarget(),c)
    card::is_type(d, c, TYPE_EQUIP)
        && (!card::is_original_type(d, c, TYPE_MONSTER) || card::is_original_type(d, c, TYPE_UNION))
        && match card::get_equip_target(d, c) {
            None => false,
            Some(et) => duel::is_existing_target(d, Some(&|d, tc| tcfilter(d, tc, c)), 0, LOCATION_MZONE, LOCATION_MZONE, 1, et.into()),
        }
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.ecfilter,tp,LOCATION_SZONE,LOCATION_SZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&ecfilter), a.tp, LOCATION_SZONE, LOCATION_SZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,0))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 0));
    // local g=Duel.SelectTarget(tp,s.ecfilter,tp,LOCATION_SZONE,LOCATION_SZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&ecfilter), a.tp, LOCATION_SZONE, LOCATION_SZONE, 1, 1, false, NO_EX).nil();
    // local ec=g:GetFirst()
    let ec = group::get_first(d, &g).nil();
    // e:SetLabelObject(ec)
    effect::set_label_object(d, a.e, Value::Card(ec));
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 1));
    // local tc=Duel.SelectTarget(tp,s.tcfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,ec:GetEquipTarget(),ec)
    let ex = card::get_equip_target(d, ec).map_or(NO_EX, |t| t.into());
    let _tc = duel::select_target(d, a.tp, Some(&|d, tc| tcfilter(d, tc, ec)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, ex);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local ec=e:GetLabelObject()
    let ec = effect::get_label_object(d, a.e).as_card();
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    // local tc=g:GetFirst()
    let mut tc = group::get_first(d, &g);
    // if tc==ec then tc=g:GetNext() end
    if tc == ec {
        tc = group::take_at_pos(d, &g, 1);
    }
    // if ec:IsFaceup() and ec:IsRelateToEffect(e) then
    let ec = ec.nil();
    if card::is_faceup(d, ec) && card::is_relate_to_effect(d, ec, a.e) {
        // if tc:IsFaceup() and tc:IsRelateToEffect(e) then
        let tc = tc.nil();
        if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
            // Duel.Equip(tp,ec,tc)
            duel::equip(d, a.tp, ec, tc, true, false);
        } else {
            // Duel.SendtoGrave(ec,REASON_EFFECT)
            duel::send_to_grave(d, ec, REASON_EFFECT, None, None);
        }
    }
}
