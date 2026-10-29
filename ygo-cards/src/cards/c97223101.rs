//! That's 10!
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 97223101;
const COUNTER_ACCESS: u16 = 0x212;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], counter_place_list: &[COUNTER_ACCESS as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Can place Access Counters on it
    // c:EnableCounterPermit(COUNTER_ACCESS,LOCATION_STZONE)
    card::enable_counter_permit(d, c, COUNTER_ACCESS, Some(LOCATION_STZONE), None);
    // c:SetCounterLimit(COUNTER_ACCESS,10)
    card::set_counter_limit(d, c, COUNTER_ACCESS, 10);
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Place 1 Access Counter on this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e1, CATEGORY_COUNTER);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCondition(s.countercon)
    effect::set_condition(d, e1, Some(condition(countercon)));
    // e1:SetTarget(s.countertg)
    effect::set_target(d, e1, Some(cost_target(countertg)));
    // e1:SetOperation(s.counterop)
    effect::set_operation(d, e1, Some(operation(counterop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Monsters you control gain 100 ATK for each Access Counter on this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetValue(function(e,c) return e:GetHandler():GetCounter(COUNTER_ACCESS)*100 end)
    effect::set_value_fn(d, e2, effect_value(atk_value));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Return this card to the hand, and if you do, Set 1 Continuous Trap with an effect that Special Summons itself directly from your Deck
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e3, CATEGORY_TOHAND);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_ADD_COUNTER+COUNTER_ACCESS)
    effect::set_code(d, e3, EVENT_ADD_COUNTER + COUNTER_ACCESS as u32);
    // e3:SetCondition(function(e) return e:GetHandler():GetCounter(COUNTER_ACCESS)==10 end)
    effect::set_condition(d, e3, Some(condition(counter10)));
    // e3:SetTarget(s.settg)
    effect::set_target(d, e3, Some(cost_target(settg)));
    // e3:SetOperation(s.setop)
    effect::set_operation(d, e3, Some(operation(setop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.countercon(e,tp,eg,ep,ev,re,r,rp)
fn countercon(d: &mut Duel, a: &EffArgs) -> bool {
    // local code1,code2=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_CODE,CHAININFO_TRIGGERING_CODE2)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_CODE, CHAININFO_TRIGGERING_CODE2]);
    // return code1~=id and code2~=id
    !info.first().is_some_and(|v| v.int() == ID as i64) && !info.get(1).is_some_and(|v| v.int() == ID as i64)
}
// function s.countertg(e,tp,eg,ep,ev,re,r,rp,chk)
fn countertg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():IsCanAddCounter(COUNTER_ACCESS,1) end
    if a.chk == Some(0) {
        return card::is_can_add_counter(d, c, COUNTER_ACCESS, 1, false, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_COUNTER,nil,1,tp,COUNTER_ACCESS)
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, None, 1, tp, COUNTER_ACCESS as i32);
    true
}
// function s.counterop(e,tp,eg,ep,ev,re,r,rp)
fn counterop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // c:AddCounter(COUNTER_ACCESS,1)
        card::add_counter(d, c, COUNTER_ACCESS, 1, false);
    }
}
// function s.setfilter(c,ignore_zones)
fn setfilter(d: &mut Duel, c: CardId, ignore_zones: bool) -> bool {
    // return c:IsContinuousTrap() and c:IsTrapMonster() and c:IsSSetable(ignore_zones)
    card::is_continuous_trap(d, c) && card::is_trap_monster(d, c) && card::is_s_setable(d, c, ignore_zones, None)
}
// function s.settg(e,tp,eg,ep,ev,re,r,rp,chk)
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToHand() and Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_DECK,0,1,nil,true) end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None)
            && duel::is_existing_matching_card(d, Some(&|d, x| setfilter(d, x, true)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
// function s.setop(e,tp,eg,ep,ev,re,r,rp)
fn setop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not (c:IsRelateToEffect(e) and Duel.SendtoHand(c,nil,REASON_EFFECT)>0 and c:IsLocation(LOCATION_HAND)) then return end
    if !(card::is_relate_to_effect(d, c, a.e)
        && duel::send_to_hand(d, c, None, REASON_EFFECT, None) > 0
        && card::is_location(d, c, LOCATION_HAND))
    {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.setfilter,tp,LOCATION_DECK,0,1,1,nil,false):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, x| setfilter(d, x, false)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if sc and Duel.SSet(tp,sc)>0 then
    if let Some(sc) = group::get_first(d, &g) {
        if duel::s_set(d, tp, sc, None, true) > 0 {
            // Can be activated this turn
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetDescription(aux.Stringid(id,2))
            effect::set_description(d, e1, aux::stringid(ID, 2));
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
            effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE, 0);
            // e1:SetCode(EFFECT_TRAP_ACT_IN_SET_TURN)
            effect::set_code(d, e1, EFFECT_TRAP_ACT_IN_SET_TURN);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // sc:RegisterEffect(e1)
            card::register_effect(d, sc, e1, false);
        }
    }
}
// function(e,c) return e:GetHandler():GetCounter(COUNTER_ACCESS)*100 end
fn atk_value(d: &mut Duel, e: EffectId) -> i64 {
    let h = effect::get_handler(d, e).nil();
    card::get_counter(d, h, COUNTER_ACCESS) as i64 * 100
}
// function(e) return e:GetHandler():GetCounter(COUNTER_ACCESS)==10 end
fn counter10(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::get_counter(d, h, COUNTER_ACCESS) == 10
}
