//! Borrel Cooling
use super::prelude::*;

pub const ID: u32 = 62753201;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_series={SET_ROKKET,SET_BORREL}
    listed_series: &[SET_ROKKET, SET_BORREL],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.cfilter(c,tp)
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_ROKKET) and Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,0,1,c)
    card::is_set_card(d, c, &[SET_ROKKET as u16]) && duel::is_existing_target(d, Some(&tgfilter), tp, LOCATION_MZONE, 0, 1, c.into())
}
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.cfilter,1,false,nil,nil,tp) end
    if a.chk == Some(0) {
        return aux::check_release_group_cost(d, tp, Some(&|d, c| cfilter(d, c, tp)), 1, 1, false, None, NO_EX);
    }
    // local rg=Duel.SelectReleaseGroupCost(tp,s.cfilter,1,1,false,nil,nil,tp)
    let rg = aux::select_release_group_cost(d, tp, Some(&|d, c| cfilter(d, c, tp)), 1, 1, false, None, NO_EX);
    // Duel.Release(rg,REASON_COST)
    duel::release(d, rg, REASON_COST, None);
    // aux.RemainFieldCost(e,tp,eg,ep,ev,re,r,rp,1)
    let args = [Value::Effect(a.e), Value::Int(tp as i64), Value::Nil, Value::Nil, Value::Nil, Value::Nil, Value::Nil, Value::Nil, Value::Int(1)];
    aux::remain_field_cost().call(d, &args);
    true
}
// function s.tgfilter(c)
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsLinkMonster() and c:IsSetCard(SET_BORREL)
    card::is_faceup(d, c) && card::is_link_monster(d, c) && card::is_set_card(d, c, &[SET_BORREL as u16])
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.tgfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && tgfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tgfilter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    // Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, tp, Some(&tgfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,e:GetHandler(),1,0,0)
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
// function s.eqlimit(e,c)
fn eqlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:GetControler()==e:GetHandlerPlayer() and c:IsType(TYPE_LINK) and c:IsSetCard(SET_BORREL)
    let hp = effect::get_handler_player(d, e);
    card::is_controler(d, c, hp) && card::is_type(d, c, TYPE_LINK) && card::is_set_card(d, c, &[SET_BORREL as u16])
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsLocation(LOCATION_SZONE) or not c:IsRelateToEffect(e) or c:IsStatus(STATUS_LEAVE_CONFIRMED) then return end
    if !card::is_location(d, c, LOCATION_SZONE) || !card::is_relate_to_effect(d, c, a.e) || card::is_status(d, c, STATUS_LEAVE_CONFIRMED) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
            // local fid=c:GetFieldID()
            let fid = card::get_field_id(d, c);
            // Duel.Equip(tp,c,tc)
            duel::equip(d, tp, c, tc, true, false);
            // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1,fid)
            card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, fid as i64, 0);
            // Equip limit
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_EQUIP_LIMIT)
            effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetValue(s.eqlimit)
            effect::set_value_fn(d, e1, effect_card_fn(|d, e, c| eqlimit(d, e, c)));
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
            // Granted effect
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetDescription(aux.Stringid(id,0))
            effect::set_description(d, e2, aux::stringid(ID, 0));
            // e2:SetType(EFFECT_TYPE_QUICK_O)
            effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
            // e2:SetCode(EVENT_FREE_CHAIN)
            effect::set_code(d, e2, EVENT_FREE_CHAIN);
            // e2:SetRange(LOCATION_MZONE)
            effect::set_range(d, e2, LOCATION_MZONE);
            // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
            effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
            // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
            // e2:SetCountLimit(1)
            effect::set_count_limit(d, e2, 1, 0, 0, 0);
            // e2:SetTarget(s.indtg)
            effect::set_target(d, e2, Some(cost_target(indtg)));
            // e2:SetOperation(s.indop)
            effect::set_operation(d, e2, Some(operation(indop)));
            // local e3=Effect.CreateEffect(c)
            let e3 = effect::create_effect(d, c);
            // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_GRANT)
            effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_GRANT);
            // e3:SetRange(LOCATION_SZONE)
            effect::set_range(d, e3, LOCATION_SZONE);
            // e3:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
            effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
            // e3:SetTarget(s.eftg)
            effect::set_target(d, e3, Some(effect_card_fn(|d, e, c| eftg(d, e, c))));
            // e3:SetLabelObject(e2)
            effect::set_label_object(d, e3, Value::Effect(e2));
            // c:RegisterEffect(e3)
            card::register_effect(d, c, e3, false);
        } else {
            // c:CancelToGrave(false)
            card::cancel_to_grave(d, c, false);
        }
    } else {
        // c:CancelToGrave(false)
        card::cancel_to_grave(d, c, false);
    }
}
// function s.eftg(e,c)
fn eftg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return e:GetHandler():GetEquipTarget()==c
    let h = effect::get_handler(d, e).nil();
    card::get_equip_target(d, h) == Some(c)
}
// function s.indtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn indtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && card::is_faceup(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsFaceup,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,Card.IsFaceup,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, tp, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
// function s.indop(e,tp,eg,ep,ev,re,r,rp)
fn indop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
            // Cannot be destroyed by battle or card effect
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetDescription(3008)
            effect::set_description(d, e1, 3008);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
            effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
            // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetValue(1)
            effect::set_value(d, e1, 1);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
            effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
        }
    }
}
