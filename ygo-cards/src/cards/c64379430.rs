//! 終焉の精霊 (Doomsday Horror)
use super::prelude::*;

pub const ID: u32 = 64379430;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_SET_ATTACK)
    effect::set_code(d, e1, EFFECT_SET_ATTACK);
    // e1:SetValue(s.value)
    effect::set_value_fn(d, e1, effect_card_value(value));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EFFECT_SET_DEFENSE)
    effect::set_code(d, e2, EFFECT_SET_DEFENSE);
    // e2:SetValue(s.value)
    effect::set_value_fn(d, e2, effect_card_value(value));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // return
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e3, CATEGORY_TOGRAVE);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetCondition(s.retcon)
    effect::set_condition(d, e3, Some(condition(retcon)));
    // e3:SetTarget(s.rettg)
    effect::set_target(d, e3, Some(cost_target(rettg)));
    // e3:SetOperation(s.retop)
    effect::set_operation(d, e3, Some(operation(retop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAttribute(ATTRIBUTE_DARK)
    card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_DARK)
}
// function s.value(e,c)
fn value(d: &mut Duel, _e: EffectId, _c: CardId) -> i64 {
    // return Duel.GetMatchingGroupCount(s.filter,0,LOCATION_REMOVED,LOCATION_REMOVED,nil)*300
    duel::get_matching_group_count(d, Some(&filter), 0, LOCATION_REMOVED, LOCATION_REMOVED, NO_EX) as i64 * 300
}
// function s.retcon(e,tp,eg,ep,ev,re,r,rp)
fn retcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsReason(REASON_DESTROY)
    let c = effect::get_handler(d, a.e).nil();
    card::is_reason(d, c, REASON_DESTROY)
}
// function s.rettg(e,tp,eg,ep,ev,re,r,rp,chk)
fn rettg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(s.filter,0,LOCATION_REMOVED,LOCATION_REMOVED,nil)
    let g = duel::get_matching_group(d, Some(&filter), 0, LOCATION_REMOVED, LOCATION_REMOVED, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(g), n, 0, 0);
    true
}
// function s.retop(e,tp,eg,ep,ev,re,r,rp)
fn retop(d: &mut Duel, _a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.filter,0,LOCATION_REMOVED,LOCATION_REMOVED,nil)
    let g = duel::get_matching_group(d, Some(&filter), 0, LOCATION_REMOVED, LOCATION_REMOVED, NO_EX);
    // Duel.SendtoGrave(g,REASON_EFFECT|REASON_RETURN)
    duel::send_to_grave(d, g, REASON_EFFECT | REASON_RETURN, None, None);
}
