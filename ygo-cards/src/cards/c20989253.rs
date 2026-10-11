//! Amazement Family Faces
use super::prelude::*;

pub const ID: u32 = 20989253;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_AMAZEMENT, SET_ATTRACTION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Equip this card to 1 monster
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetCategory(CATEGORY_CONTROL+CATEGORY_EQUIP)
    effect::set_category(d, e0, CATEGORY_CONTROL + CATEGORY_EQUIP);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // e0:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e0, EFFECT_FLAG_CARD_TARGET, 0);
    // e0:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e0, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e0:SetCost(aux.RemainFieldCost)
    effect::set_cost(d, e0, Some(aux::remain_field_cost()));
    // e0:SetTarget(s.target)
    effect::set_target(d, e0, Some(cost_target(target)));
    // e0:SetOperation(s.activate)
    effect::set_operation(d, e0, Some(operation(activate)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // atk up
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(lua_fn(cond_fn)));
    // e1:SetValue(500)
    effect::set_value(d, e1, 500);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // cannot trigger
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_CANNOT_TRIGGER)
    effect::set_code(d, e2, EFFECT_CANNOT_TRIGGER);
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(lua_fn(cond_fn)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // change archetype
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
    // e3:SetCode(EFFECT_ADD_SETCODE)
    effect::set_code(d, e3, EFFECT_ADD_SETCODE);
    // e3:SetValue(SET_AMAZEMENT)
    effect::set_value(d, e3, SET_AMAZEMENT as i64);
    // e3:SetCondition(s.condition)
    effect::set_condition(d, e3, Some(lua_fn(cond_fn)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // control
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e4, EFFECT_TYPE_EQUIP);
    // e4:SetCode(EFFECT_SET_CONTROL)
    effect::set_code(d, e4, EFFECT_SET_CONTROL);
    // e4:SetValue(function(e)return e:GetHandlerPlayer()end)
    effect::set_value_fn(d, e4, lua_fn(control_value));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// s.eqcfilter(c,tp)
fn eqcfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_ATTRACTION) and c:IsTrap() and c:IsControler(tp)
    card::is_set_card(d, c, &[SET_ATTRACTION as u16]) && card::is_trap(d, c) && card::is_controler(d, c, tp)
}
// s.eqfilter(c,e,tp)
fn eqfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return aux.CheckStealEquip(c,e,tp) and c:GetEquipGroup():IsExists(s.eqcfilter,1,nil,tp)
    if !aux::check_steal_equip(d, c, e, tp) {
        return false;
    }
    let eg = card::get_equip_group(d, c);
    group::is_exists(d, &eg, &|d, x| eqcfilter(d, x, tp), 1, NO_EX)
}
// s.eqlimit(e,c)
fn eqlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return e:GetHandlerPlayer()~=c:GetControler() or e:GetHandler():GetEquipTarget()==c
    if effect::get_handler_player(d, e) != card::get_controler(d, c) {
        return true;
    }
    let h = effect::get_handler(d, e).nil();
    card::get_equip_target(d, h) == Some(c)
}
// s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.eqfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && eqfilter(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return e:IsHasType(EFFECT_TYPE_ACTIVATE)
    //     and Duel.IsExistingTarget(s.eqfilter,tp,0,LOCATION_MZONE,1,nil,e,tp) end
    if a.chk == Some(0) {
        return effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE)
            && duel::is_existing_target(d, Some(&|d, c| eqfilter(d, c, a.e, a.tp)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // Duel.SelectTarget(tp,s.eqfilter,tp,0,LOCATION_MZONE,1,1,nil,e,tp)
    let _ = duel::select_target(d, a.tp, Some(&|d, c| eqfilter(d, c, a.e, a.tp)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(h)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,tc,1,0,0)
    // (tc is a global that is never assigned: nil)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, None, 1, 0, 0);
    true
}
// s.activate(e,tp,eg,ep,ev,re,r,rp)
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
    if let Some(tc) = tc.filter(|&tc| card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc)) {
        // Duel.Equip(tp,c,tc)
        duel::equip(d, tp, c, tc, true, false);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_EQUIP_LIMIT)
        effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetValue(s.eqlimit)
        effect::set_value_fn(d, e1, effect_card_fn(eqlimit));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    } else {
        // c:CancelToGrave(false)
        card::cancel_to_grave(d, c, false);
    }
}
// s.condition(e): function(e) return e:GetHandler():GetControler()==e:GetHandlerPlayer() end
fn cond_fn(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let h = effect::get_handler(d, e).nil();
    card::get_controler(d, h) == effect::get_handler_player(d, e)
}
// function(e)return e:GetHandlerPlayer()end
fn control_value(d: &mut Duel, a: Args) -> i64 {
    effect::get_handler_player(d, a.e(0)) as i64
}
