//! Okiku's Dish Count
use super::prelude::*;

pub const ID: u32 = 89086647;
/// `local COUNTER_DISH=0x216`
const COUNTER_DISH: u16 = 0x216;

pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[COUNTER_DISH as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(COUNTER_DISH)
    card::enable_counter_permit(d, c, COUNTER_DISH, None, None);
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Place Dish Counters on this card equal to the Chain Link number of this effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e1, CATEGORY_COUNTER);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCountLimit(1,0,EFFECT_COUNT_CODE_CHAIN)
    effect::set_count_limit(d, e1, 1, 0, 0, EFFECT_COUNT_CODE_CHAIN as u8);
    // e1:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return rp==1-tp end)
    effect::set_condition(d, e1, Some(condition(cond1)));
    // e1:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk) if chk==0 then return true end
    //     Duel.SetOperationInfo(0,CATEGORY_COUNTER,e:GetHandler(),Duel.GetCurrentChain(),tp,COUNTER_DISH) end)
    effect::set_target(d, e1, Some(cost_target(target1)));
    // e1:SetOperation(function(e) e:GetHandler():AddCounter(COUNTER_DISH,Duel.GetCurrentChain()) end)
    effect::set_operation(d, e1, Some(operation(op1)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // 9 or less: Your opponent cannot target this card with card effects, also it cannot be destroyed by your opponent's card effects
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2a, EFFECT_TYPE_SINGLE);
    // e2a:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2a, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2a:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e2a, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e2a:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2a, LOCATION_SZONE);
    // e2a:SetCondition(function(e) return e:GetHandler():GetCounter(COUNTER_DISH)<=9 end)
    effect::set_condition(d, e2a, Some(condition(cond2a)));
    // e2a:SetValue(aux.tgoval)
    effect::set_value_fn(d, e2a, aux::tgoval());
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e2b=e2a:Clone()
    let e2b = effect::clone(d, e2a);
    // e2b:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e2b, EFFECT_INDESTRUCTABLE_EFFECT);
    // e2b:SetValue(aux.indoval)
    effect::set_value_fn(d, e2b, aux::indoval());
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
    // 10 or more: Send this card to the GY
    // local e2c=Effect.CreateEffect(c)
    let e2c = effect::create_effect(d, c);
    // e2c:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2c, EFFECT_TYPE_SINGLE);
    // e2c:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2c, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2c:SetCode(EFFECT_SELF_TOGRAVE)
    effect::set_code(d, e2c, EFFECT_SELF_TOGRAVE);
    // e2c:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2c, LOCATION_SZONE);
    // e2c:SetCondition(function(e) return e:GetHandler():GetCounter(COUNTER_DISH)>=10 end)
    effect::set_condition(d, e2c, Some(condition(cond2c)));
    // c:RegisterEffect(e2c)
    card::register_effect(d, c, e2c, false);
    // Send the top 10 cards of your Deck to the GY
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DECKDES)
    effect::set_category(d, e3, CATEGORY_DECKDES);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return e:GetHandler():IsReason(REASON_EFFECT) and rp==PLAYER_NONE end)
    effect::set_condition(d, e3, Some(condition(cond3)));
    // e3:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk) if chk==0 then return Duel.IsPlayerCanDiscardDeck(tp,10) end
    //     Duel.SetOperationInfo(0,CATEGORY_DECKDES,nil,0,tp,10) end)
    effect::set_target(d, e3, Some(cost_target(target3)));
    // e3:SetOperation(function(e,tp) Duel.DiscardDeck(tp,10,REASON_EFFECT) end)
    effect::set_operation(d, e3, Some(operation(op3)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return rp==1-tp end`
fn cond1(_d: &mut Duel, a: &EffArgs) -> bool {
    a.rp == 1 - a.tp
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk)` (the activation target of e1)
fn target1(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_COUNTER,e:GetHandler(),Duel.GetCurrentChain(),tp,COUNTER_DISH)
    let h = effect::get_handler(d, a.e).nil();
    let chain = duel::get_current_chain(d, false);
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, Some(CardSet::from_card(h)), chain as u8, a.tp, COUNTER_DISH as i32);
    true
}
/// `function(e) e:GetHandler():AddCounter(COUNTER_DISH,Duel.GetCurrentChain()) end`
fn op1(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    let chain = duel::get_current_chain(d, false);
    card::add_counter(d, h, COUNTER_DISH, chain as u16, false);
}
/// `function(e) return e:GetHandler():GetCounter(COUNTER_DISH)<=9 end`
fn cond2a(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::get_counter(d, h, COUNTER_DISH) <= 9
}
/// `function(e) return e:GetHandler():GetCounter(COUNTER_DISH)>=10 end`
fn cond2c(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::get_counter(d, h, COUNTER_DISH) >= 10
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return e:GetHandler():IsReason(REASON_EFFECT) and rp==PLAYER_NONE end`
fn cond3(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_reason(d, h, REASON_EFFECT) && a.rp == PLAYER_NONE as u8
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk) ... end` (the target of e3)
fn target3(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDiscardDeck(tp,10) end
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck(d, a.tp, 10);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DECKDES,nil,0,tp,10)
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, a.tp, 10);
    true
}
/// `function(e,tp) Duel.DiscardDeck(tp,10,REASON_EFFECT) end`
fn op3(d: &mut Duel, a: &EffArgs) {
    duel::discard_deck(d, a.tp, 10, REASON_EFFECT);
}
