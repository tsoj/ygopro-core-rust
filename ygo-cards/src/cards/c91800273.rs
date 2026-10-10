//! Dimension Shifter
//! scripted by Logical Nonsense
//! Substitute ID
use super::prelude::*;

pub const ID: u32 = 91800273;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Cards sent to GY are banished instead
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetCost(Cost.SelfToGrave)
    effect::set_cost(d, e1, Some(cost::self_to_grave()));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e)`
//  If no cards in your GY
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(e:GetHandlerPlayer(),LOCATION_GRAVE,0)==0
    let p = effect::get_handler_player(d, a.e);
    duel::get_field_group_count(d, p, LOCATION_GRAVE, 0) == 0
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
//  Send this card from hand to GY
//  Cards sent to GY are banished instead, lasts to next turn
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_SET_AVAILABLE+EFFECT_FLAG_IGNORE_RANGE+EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE + EFFECT_FLAG_IGNORE_RANGE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetCode(EFFECT_TO_GRAVE_REDIRECT)
    effect::set_code(d, e1, EFFECT_TO_GRAVE_REDIRECT);
    // e1:SetTargetRange(0xff,0xff)
    effect::set_target_range(d, e1, 0xff, 0xff);
    // e1:SetReset(RESET_PHASE|PHASE_END,2)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 2);
    // e1:SetValue(LOCATION_REMOVED)
    effect::set_value(d, e1, LOCATION_REMOVED as i64);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
