//! Amaze Attraction Thrill Train
use super::prelude::*;

pub const ID: u32 = 36591747;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_AMAZEMENT, SET_ATTRACTION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // From cards_specific_functions.lua
    // aux.AddAttractionEquipProc(c)
    aux::add_attraction_equip_proc(d, c);
    // You: Change battle position and Set 1 "Attraction" Trap from your GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_POSITION+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_POSITION + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetCondition(aux.AttractionEquipCon(true))
    effect::set_condition(d, e1, Some(aux::attraction_equip_con(true)));
    // e1:SetTarget(s.settg)
    effect::set_target(d, e1, Some(cost_target(settg)));
    // e1:SetOperation(s.setop)
    effect::set_operation(d, e1, Some(operation(setop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Your opponent: Banish the equipped monster until the End Phase
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_REMOVE);
    // e2:SetHintTiming(0)
    effect::set_hint_timing(d, e2, 0, None);
    // e2:SetCondition(aux.AttractionEquipCon(false))
    effect::set_condition(d, e2, Some(aux::attraction_equip_con(false)));
    // e2:SetTarget(s.rmtg)
    effect::set_target(d, e2, Some(cost_target(rmtg)));
    // e2:SetOperation(s.rmop)
    effect::set_operation(d, e2, Some(operation(rmop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ATTRACTION) and c:IsTrap() and c:IsSSetable()
    card::is_set_card(d, c, &[SET_ATTRACTION as u16]) && card::is_trap(d, c) && card::is_s_setable(d, c, false, None)
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local tc=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let tc = card::get_equip_target(d, h);
    // if chk==0 then return tc and tc:IsCanChangePosition() and Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return tc.is_some_and(|tc| card::is_can_change_position(d, tc))
            && duel::is_existing_matching_card(d, Some(&setfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, tc.map(CardSet::from_card), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, None, 1, tp, 0);
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let tc = card::get_equip_target(d, h);
    // if tc and tc:IsLocation(LOCATION_MZONE) and Duel.ChangePosition(tc,POS_FACEUP_DEFENSE,POS_FACEDOWN_DEFENSE,POS_FACEUP_ATTACK,POS_FACEUP_ATTACK)>0 then
    if let Some(tc) = tc {
        if card::is_location(d, tc, LOCATION_MZONE)
            && duel::change_position(d, tc, POS_FACEUP_DEFENSE as u8, Some(POS_FACEDOWN_DEFENSE as u8), Some(POS_FACEUP_ATTACK as u8), Some(POS_FACEUP_ATTACK as u8), false) > 0
        {
            // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_SET)
            duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_SET as u64);
            // local g=Duel.SelectMatchingCard(1-tp,s.setfilter,tp,LOCATION_GRAVE,0,1,1,nil)
            let g = duel::select_matching_card(d, 1 - tp, Some(&setfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
            // if #g>0 then Duel.SSet(tp,g) end
            if !g.is_empty() {
                duel::s_set(d, tp, g, None, true);
            }
        }
    }
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let tc = card::get_equip_target(d, h);
    // if chk==0 then return tc and tc:IsAbleToRemove() end
    if a.chk == Some(0) {
        return tc.is_some_and(|tc| card::is_able_to_remove(d, tc, None, None, None));
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, tc.map(CardSet::from_card), 1, 0, 0);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let tc = card::get_equip_target(d, h);
    // if tc and tc:IsLocation(LOCATION_MZONE) and Duel.Remove(tc,0,REASON_EFFECT|REASON_TEMPORARY)>0 then
    let Some(tc) = tc else { return };
    if !card::is_location(d, tc, LOCATION_MZONE) || duel::remove(d, tc, 0, REASON_EFFECT | REASON_TEMPORARY, None, None) <= 0 {
        return;
    }
    // tc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
    card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    // Return it in the End Phase
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetLabelObject(tc)
    effect::set_label_object(d, e1, Value::Card(tc));
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.retcon)
    effect::set_condition(d, e1, Some(condition(retcon)));
    // e1:SetOperation(s.retop)
    effect::set_operation(d, e1, Some(operation(retop)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// `s.retcon(e,tp,eg,ep,ev,re,r,rp)`
fn retcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetLabelObject():GetFlagEffect(id)>0
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    card::get_flag_effect(d, tc, ID) > 0
}
/// `s.retop(e,tp,eg,ep,ev,re,r,rp)`
fn retop(d: &mut Duel, a: &EffArgs) {
    // Duel.ReturnToField(e:GetLabelObject())
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    duel::return_to_field(d, tc, None, None);
}
