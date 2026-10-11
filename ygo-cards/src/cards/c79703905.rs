//! Cloudian - Altus
use super::prelude::*;

pub const ID: u32 = 79703905;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CLOUDIAN], counter_place_list: &[COUNTER_FOG], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // battle indestructable
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // selfdes
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EFFECT_SELF_DESTROY)
    effect::set_code(d, e2, EFFECT_SELF_DESTROY);
    // e2:SetCondition(s.sdcon)
    effect::set_condition(d, e2, Some(condition(sdcon)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // counter
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e3, CATEGORY_COUNTER);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SUMMON_SUCCESS);
    // e3:SetOperation(s.addc)
    effect::set_operation(d, e3, Some(operation(addc)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // handes
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_HANDES)
    effect::set_category(d, e4, CATEGORY_HANDES);
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCost(s.hdcost)
    effect::set_cost(d, e4, Some(cost_target(hdcost)));
    // e4:SetTarget(s.hdtg)
    effect::set_target(d, e4, Some(cost_target(hdtg)));
    // e4:SetOperation(s.hdop)
    effect::set_operation(d, e4, Some(operation(hdop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.sdcon(e)`
fn sdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsPosition(POS_FACEUP_DEFENSE)
    let c = effect::get_handler(d, a.e).nil();
    card::is_position(d, c, POS_FACEUP_DEFENSE)
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_CLOUDIAN)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_CLOUDIAN as u16])
}
/// `s.addc(e,tp,eg,ep,ev,re,r,rp)`
fn addc(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():IsRelateToEffect(e) then
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        // local ct=Duel.GetMatchingGroupCount(s.cfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
        let ct = duel::get_matching_group_count(d, Some(&cfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
        // e:GetHandler():AddCounter(COUNTER_NEED_ENABLE+COUNTER_FOG,ct)
        card::add_counter(d, c, (COUNTER_NEED_ENABLE + COUNTER_FOG) as u16, ct as u16, false);
    }
}
/// `s.hdcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn hdcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsCanRemoveCounter(tp,1,1,COUNTER_FOG,3,REASON_COST) end
    if a.chk == Some(0) {
        return duel::is_can_remove_counter(d, a.tp, 1, 1, COUNTER_FOG as u16, 3, REASON_COST);
    }
    // Duel.RemoveCounter(tp,1,1,COUNTER_FOG,3,REASON_COST)
    duel::remove_counter(d, a.tp, 1, 1, COUNTER_FOG as u16, 3, REASON_COST);
    true
}
/// `s.hdtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn hdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)~=0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND) != 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, a.tp, 1);
    true
}
/// `s.hdop(e,tp,eg,ep,ev,re,r,rp)`
fn hdop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_HAND):RandomSelect(tp,1)
    let all = duel::get_field_group(d, a.tp, 0, LOCATION_HAND);
    let g = group::random_select(d, &all, a.tp, 1);
    // Duel.SendtoGrave(g,REASON_EFFECT|REASON_DISCARD)
    duel::send_to_grave(d, g, REASON_EFFECT | REASON_DISCARD, None, None);
}
