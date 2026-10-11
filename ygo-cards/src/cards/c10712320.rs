//! Vylon Stigma
use super::prelude::*;

pub const ID: u32 = 10712320;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VYLON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --equip
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTarget(s.eqtg)
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    // e1:SetOperation(s.eqop)
    effect::set_operation(d, e1, Some(operation(eqop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter1(c,tp)`
fn filter1(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_VYLON) and c:GetEquipTarget()
    //   and Duel.IsExistingTarget(s.filter2,tp,LOCATION_MZONE,LOCATION_MZONE,1,c:GetEquipTarget(),c)
    if !card::is_faceup(d, c) || !card::is_set_card(d, c, &[SET_VYLON as u16]) {
        return false;
    }
    let et = card::get_equip_target(d, c);
    if et.is_none() {
        return false;
    }
    let ex: Exclude = et.map_or(NO_EX, |x| x.into());
    duel::is_existing_target(d, Some(&|d, x| filter2(d, x, c)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, ex)
}
/// `s.filter2(c,eqc)`
fn filter2(d: &mut Duel, c: CardId, eqc: CardId) -> bool {
    // return c:IsFaceup() and eqc:CheckEquipTarget(c)
    card::is_faceup(d, c) && card::check_equip_target(d, eqc, c)
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter1,tp,LOCATION_SZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter1(d, c, a.tp)), a.tp, LOCATION_SZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // local g1=Duel.SelectTarget(tp,s.filter1,tp,LOCATION_SZONE,0,1,1,nil,tp)
    let g1 = duel::select_target(d, a.tp, Some(&|d, c| filter1(d, c, a.tp)), a.tp, LOCATION_SZONE, 0, 1, 1, false, NO_EX).nil();
    // local eqc=g1:GetFirst()
    let eqc = group::get_first(d, &g1).nil();
    // e:SetLabelObject(eqc)
    effect::set_label_object(d, a.e, Value::Card(eqc));
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // local g2=Duel.SelectTarget(tp,s.filter2,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,eqc:GetEquipTarget(),eqc)
    let et = card::get_equip_target(d, eqc);
    let ex: Exclude = et.map_or(NO_EX, |x| x.into());
    let _g2 = duel::select_target(d, a.tp, Some(&|d, c| filter2(d, c, eqc)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, ex);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,g1,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(g1), 1, 0, 0);
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    // local eqc=e:GetLabelObject()
    let eqc = effect::get_label_object(d, a.e).as_card().nil();
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]).remove(0).as_group().cloned().nil();
    // local tc=g:GetFirst()
    let order = group::iter(d, &g);
    let first = order.first().copied();
    // if tc==eqc then tc=g:GetNext() end
    let tc = if first == Some(eqc) { order.get(1).copied() } else { first };
    // if not eqc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, eqc, a.e) {
        return;
    }
    // Duel.Equip(tp,eqc,tc)
    duel::equip(d, a.tp, eqc, tc.nil(), true, false);
}
