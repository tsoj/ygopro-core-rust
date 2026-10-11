//! Amazement Precious Park
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 33773528;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ATTRACTION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // "Attraction" Trap cards can be activated during the turn they are set (during your Main Phase)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_TRAP_ACT_IN_SET_TURN)
    effect::set_code(d, e2, EFFECT_TRAP_ACT_IN_SET_TURN);
    // e2:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e2, EFFECT_FLAG_SET_AVAILABLE, 0);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetTargetRange(LOCATION_SZONE,0)
    effect::set_target_range(d, e2, LOCATION_SZONE, 0);
    // e2:SetCondition(function(e) return Duel.IsMainPhase() and Duel.IsTurnPlayer(e:GetHandlerPlayer()) end)
    effect::set_condition(d, e2, Some(condition(main_phase_cond)));
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_ATTRACTION))
    effect::set_target(d, e2, Some(effect_card_fn(attraction_target)));
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Set 1 "Attraction" Trap that is banished or in the GY
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_SET)
    effect::set_category(d, e3, CATEGORY_SET);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCost(s.setcost)
    effect::set_cost(d, e3, Some(cost_target(setcost)));
    // e3:SetTarget(s.settg)
    effect::set_target(d, e3, Some(cost_target(settg)));
    // e3:SetOperation(s.setop)
    effect::set_operation(d, e3, Some(operation(setop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(e) return Duel.IsMainPhase() and Duel.IsTurnPlayer(e:GetHandlerPlayer()) end`
fn main_phase_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_main_phase(d, None) && duel::is_turn_player(d, effect::get_handler_player(d, a.e))
}
/// `aux.TargetBoolFunction(Card.IsSetCard,SET_ATTRACTION)`
fn attraction_target(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_ATTRACTION as u16])
}
/// `s.setcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn setcost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // return true
    true
}
/// `s.eqfilter(c,ec)`
fn eqfilter(d: &mut Duel, c: CardId, ec: CardId) -> bool {
    // return c:GetEquipGroup():IsContains(ec)
    let eg = card::get_equip_group(d, c);
    group::is_contains(&eg, ec)
}
/// `s.setfilter(c,ec)`; `ec` is the Lua label object (nil when unset)
fn setfilter(d: &mut Duel, c: CardId, ec: Option<CardId>) -> bool {
    // return c:IsTrap() and c:IsSetCard(SET_ATTRACTION) and (c:IsFaceup() or c:IsLocation(LOCATION_GRAVE))
    //     and not c:IsCode(ec:GetCode()) and c:IsSSetable(true)
    if !(card::is_trap(d, c)
        && card::is_set_card(d, c, &[SET_ATTRACTION as u16])
        && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_GRAVE)))
    {
        return false;
    }
    let codes = card::get_code(d, ec.nil());
    !card::is_code(d, c, &codes) && card::is_s_setable(d, c, true, None)
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsTrap() and c:IsSetCard(SET_ATTRACTION)
    //     and Duel.IsExistingMatchingCard(s.eqfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil,c)
    //     and Duel.IsExistingTarget(s.setfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,nil,c)
    if !(card::is_faceup(d, c) && card::is_trap(d, c) && card::is_set_card(d, c, &[SET_ATTRACTION as u16])) {
        return false;
    }
    duel::is_existing_matching_card(d, Some(&|d, x| eqfilter(d, x, c)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
        && duel::is_existing_target(d, Some(&|d, x| setfilter(d, x, Some(c))), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX)
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE|LOCATION_REMOVED) and chkc:IsControler(tp) and s.setfilter(chkc,e:GetLabelObject()) end
    if let Some(chkc) = a.chkc() {
        let label_obj = effect::get_label_object(d, a.e).as_card();
        return card::is_location(d, chkc, LOCATION_GRAVE | LOCATION_REMOVED)
            && card::is_controler(d, chkc, tp)
            && setfilter(d, chkc, label_obj);
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()~=1 then return false end
        if effect::get_label(d, a.e) != 1 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return Duel.GetLocationCount(tp,LOCATION_SZONE)>-1 and Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_SZONE,0,1,nil,tp)
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > -1
            && duel::is_existing_matching_card(d, Some(&|d, x| cfilter(d, x, tp)), tp, LOCATION_SZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local gc=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_SZONE,0,1,1,nil,tp):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, x| cfilter(d, x, tp)), tp, LOCATION_SZONE, 0, 1, 1, false, NO_EX).nil();
    let gc = group::get_first(d, &sel).nil();
    // e:SetLabelObject(gc)
    effect::set_label_object(d, a.e, Value::Card(gc));
    // Duel.SendtoGrave(gc,REASON_COST)
    duel::send_to_grave(d, gc, REASON_COST, None, None);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    // local g=Duel.SelectTarget(tp,s.setfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,1,nil,gc)
    let g = duel::select_target(d, tp, Some(&|d, x| setfilter(d, x, Some(gc))), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, g, 1, 0, 0);
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsSSetable() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_s_setable(d, tc, false, None) {
        // Duel.SSet(tp,tc)
        duel::s_set(d, a.tp, tc, None, true);
    }
}
