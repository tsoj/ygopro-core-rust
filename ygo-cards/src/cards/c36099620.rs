//! Realm of Light
use super::prelude::*;

pub const ID: u32 = 36099620;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_LIGHTSWORN], counter_place_list: &[0x5], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(0x5)
    card::enable_counter_permit(d, c, 0x5, None, None);
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add counter
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetOperation(s.acop)
    effect::set_operation(d, e2, Some(operation(acop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // atkup
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
    // e3:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_LIGHTSWORN))
    effect::set_target(d, e3, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_LIGHTSWORN as u16]))));
    // e3:SetValue(s.atkval)
    effect::set_value_fn(d, e3, effect_value(atkval));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Destroy replace
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    // e4:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e4, EFFECT_DESTROY_REPLACE);
    // e4:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e4:SetRange(LOCATION_FZONE)
    effect::set_range(d, e4, LOCATION_FZONE);
    // e4:SetTarget(s.desreptg)
    effect::set_target(d, e4, Some(cost_target(desreptg)));
    // e4:SetOperation(s.desrepop)
    effect::set_operation(d, e4, Some(operation(desrepop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, e: EffectId) -> i64 {
    // return e:GetHandler():GetCounter(0x5)*100
    let h = effect::get_handler(d, e).nil();
    card::get_counter(d, h, 0x5) as i64 * 100
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:GetPreviousLocation()==LOCATION_DECK and c:IsPreviousControler(tp)
    card::get_previous_location(d, c) == LOCATION_DECK && card::is_previous_controler(d, c, tp)
}
/// `s.acop(e,tp,eg,ep,ev,re,r,rp)`
fn acop(d: &mut Duel, a: &EffArgs) {
    // if eg:IsExists(s.cfilter,1,nil,tp) then
    let tp = a.tp;
    if group::is_exists(d, &a.eg.clone().nil(), &|d, c| cfilter(d, c, tp), 1, NO_EX) {
        // e:GetHandler():AddCounter(0x5,1)
        let h = effect::get_handler(d, a.e).nil();
        card::add_counter(d, h, 0x5, 1, false);
    }
}
/// `s.desreptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn desreptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return not e:GetHandler():IsReason(REASON_REPLACE+REASON_RULE)
    //   and e:GetHandler():GetCounter(0x5)>=2 end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return !card::is_reason(d, h, REASON_REPLACE + REASON_RULE) && card::get_counter(d, h, 0x5) >= 2;
    }
    // return true
    true
}
/// `s.desrepop(e,tp,eg,ep,ev,re,r,rp)`
fn desrepop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():RemoveCounter(ep,0x5,2,REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    card::remove_counter(d, h, a.ep, 0x5, 2, REASON_EFFECT);
}
