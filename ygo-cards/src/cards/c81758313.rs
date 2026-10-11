//! Myriaday Dragon
//! Scripted by The Razgriz
use super::prelude::*;

pub const ID: u32 = 81758313;
/// `local COUNTER_MYRIAD=0x21d`
const COUNTER_MYRIAD: u32 = 0x21d;

pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[COUNTER_MYRIAD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(COUNTER_MYRIAD)
    card::enable_counter_permit(d, c, COUNTER_MYRIAD as u16, None, None);
    // Once per turn, during the End Phase: You can pay 100 LP; place 1 Myriad Counter on this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e1, CATEGORY_COUNTER);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(Cost.PayLP(100))
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(100))));
    // e1:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk) ... end)
    effect::set_target(d, e1, Some(cost_target(e1_target)));
    // e1:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) ... end)
    effect::set_operation(d, e1, Some(operation(e1_op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Once per turn, during your Main Phase: You can place 1 Myriad Counter on this card for every 1000 points difference between your LP and your opponent's
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e2, CATEGORY_COUNTER);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk) ... end)
    effect::set_target(d, e2, Some(cost_target(e2_target)));
    // e2:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) ... end)
    effect::set_operation(d, e2, Some(operation(e2_op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Gains 10,000 ATK/DEF while it has 10 or more Myriad Counters
    // local e3a=Effect.CreateEffect(c)
    let e3a = effect::create_effect(d, c);
    // e3a:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3a, EFFECT_TYPE_SINGLE);
    // e3a:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3a, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3a:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e3a, EFFECT_UPDATE_ATTACK);
    // e3a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3a, LOCATION_MZONE);
    // e3a:SetCondition(function(e) return e:GetHandler():GetCounter(COUNTER_MYRIAD)>=10 end)
    effect::set_condition(d, e3a, Some(lua_fn(e3a_cond)));
    // e3a:SetValue(10000)
    effect::set_value(d, e3a, 10000);
    // c:RegisterEffect(e3a)
    card::register_effect(d, c, e3a, false);
    // local e3b=e3a:Clone()
    let e3b = effect::clone(d, e3a);
    // e3b:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e3b, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e3b)
    card::register_effect(d, c, e3b, false);
    // If this card would be destroyed by battle or card effect, remove 1 Myriad Counter from it instead
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e4:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e4:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e4, EFFECT_DESTROY_REPLACE);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk) ... end)
    effect::set_target(d, e4, Some(cost_target(e4_target)));
    // e4:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) e:GetHandler():RemoveCounter(tp,COUNTER_MYRIAD,1,REASON_EFFECT) end)
    effect::set_operation(d, e4, Some(operation(e4_op)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk)` of e1
fn e1_target(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsCanAddCounter(COUNTER_MYRIAD,1) end
    if a.chk == Some(0) {
        return card::is_can_add_counter(d, c, COUNTER_MYRIAD as u16, 1, false, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_COUNTER,c,1,tp,COUNTER_MYRIAD)
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, Some(CardSet::from_card(c)), 1, a.tp, COUNTER_MYRIAD as i32);
    true
}
/// `function(e,tp,eg,ep,ev,re,r,rp)` operation of e1
fn e1_op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // c:AddCounter(COUNTER_MYRIAD,1)
        card::add_counter(d, c, COUNTER_MYRIAD as u16, 1, false);
    }
}
/// `math.abs(Duel.GetLP(tp)-Duel.GetLP(1-tp))//1000`
fn number_of_counters(d: &Duel, tp: u8) -> i64 {
    (duel::get_lp(d, tp) as i64 - duel::get_lp(d, 1 - tp) as i64).abs().div_euclid(1000)
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk)` of e2
fn e2_target(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local number_of_counters=...
    let n = number_of_counters(d, a.tp);
    // if chk==0 then return number_of_counters>0 and c:IsCanAddCounter(COUNTER_MYRIAD,number_of_counters) end
    if a.chk == Some(0) {
        return n > 0 && card::is_can_add_counter(d, c, COUNTER_MYRIAD as u16, n as u16, false, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_COUNTER,c,number_of_counters,tp,COUNTER_MYRIAD)
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, Some(CardSet::from_card(c)), n as u8, a.tp, COUNTER_MYRIAD as i32);
    true
}
/// `function(e,tp,eg,ep,ev,re,r,rp)` operation of e2
fn e2_op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local number_of_counters=...
    let n = number_of_counters(d, a.tp);
    // if number_of_counters>0 then
    if n > 0 {
        // c:AddCounter(COUNTER_MYRIAD,number_of_counters)
        card::add_counter(d, c, COUNTER_MYRIAD as u16, n as u16, false);
    }
}
/// `function(e) return e:GetHandler():GetCounter(COUNTER_MYRIAD)>=10 end` of e3a
fn e3a_cond(d: &mut Duel, a: Args) -> bool {
    let c = effect::get_handler(d, a.e(0)).nil();
    card::get_counter(d, c, COUNTER_MYRIAD as u16) >= 10
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk)` of e4
fn e4_target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // return not c:IsReason(REASON_REPLACE|REASON_RULE) and c:HasCounter(COUNTER_MYRIAD,1)
        return !card::is_reason(d, c, REASON_REPLACE | REASON_RULE) && card::has_counter(d, c, COUNTER_MYRIAD as u16);
    }
    // return true
    true
}
/// `function(e,tp,eg,ep,ev,re,r,rp)` operation of e4
fn e4_op(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    card::remove_counter(d, c, a.tp, COUNTER_MYRIAD as u16, 1, REASON_EFFECT);
}
