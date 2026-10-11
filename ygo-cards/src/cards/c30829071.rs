//! Amazement Ambassador Bufo
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 30829071;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ATTRACTION, SET_AMAZEMENT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // equip
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetTarget(s.eqtg)
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    // e1:SetOperation(s.eqop)
    effect::set_operation(d, e1, Some(operation(eqop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // From cards_specific_functions.lua
    // aux.AddAmazementQuickEquipEffect(c,id)
    aux::add_amazement_quick_equip_effect(d, c, ID);
}
fn eqfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ATTRACTION) and c:IsTrap()
    card::is_set_card(d, c, &[SET_ATTRACTION as u16]) && card::is_trap(d, c)
}
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingTarget(s.eqfilter,tp,LOCATION_GRAVE,0,1,nil)
    //     and Duel.IsExistingTarget(Card.IsFaceup,tp,0,LOCATION_MZONE,1,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&eqfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // local g1=Duel.SelectTarget(tp,s.eqfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g1 = duel::select_target(d, a.tp, Some(&eqfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // local tc=g1:GetFirst()
    let tc = group::get_first(d, &g1);
    // e:SetLabelObject(tc)
    effect::set_label_object(d, a.e, tc.map_or(Value::Nil, Value::Card));
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // local g2=Duel.SelectTarget(tp,Card.IsFaceup,tp,0,LOCATION_MZONE,1,1,nil)
    duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, tc.map(CardSet::from_card), 1, 0, 0);
    true
}
fn eqop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local ec=e:GetLabelObject()
    let ec = effect::get_label_object(d, a.e).as_card().nil();
    // local g=Duel.GetTargetCards(e)
    // if #g<2 then return end
    let g = duel::get_target_cards(d, a.e).nil();
    if g.len() < 2 {
        return;
    }
    // local tc=g:GetFirst()
    let order = group::iter(d, &g);
    let mut tc = order.first().copied();
    // if tc==ec then tc=g:GetNext() end
    if tc == Some(ec) {
        tc = order.get(1).copied();
    }
    // if ec:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, ec, a.e) && card::is_faceup(d, tc.nil()) {
        // Duel.Equip(tp,ec,tc)
        duel::equip(d, a.tp, ec, tc.nil(), true, false);
        // local e1=Effect.CreateEffect(ec)
        let e1 = effect::create_effect(d, ec);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_EQUIP_LIMIT)
        effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetValue(AA.eqlim)
        effect::set_value_fn(d, e1, aux::aa::eqlim());
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // ec:RegisterEffect(e1)
        card::register_effect(d, ec, e1, false);
    }
}
