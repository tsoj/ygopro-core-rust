//! Counter Cleaner
use super::prelude::*;

pub const ID: u32 = 38834303;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(Cost.PayLP(500))
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(500))));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.HasCounters,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::has_counters(d, c)), a.tp, LOCATION_ONFIELD as u32, LOCATION_ONFIELD as u32, 1, NO_EX);
    }
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local count=0
    let mut count: i64 = 0;
    // for tc in Duel.GetMatchingGroup(Card.HasCounters,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil):Iter() do
    let g = duel::get_matching_group(d, Some(&|d, c| card::has_counters(d, c)), a.tp, LOCATION_ONFIELD as u32, LOCATION_ONFIELD as u32, NO_EX);
    for tc in group::iter(d, &g) {
        // count=count+tc:GetCounter(COUNTER_A)
        count += card::get_counter(d, tc, COUNTER_A as u16) as i64;
        // tc:RemoveAllCounters()
        card::remove_all_counters(d, tc);
    }
    // if count>0 then
    if count > 0 {
        // Duel.RaiseEvent(e:GetHandler(),EVENT_REMOVE_COUNTER+COUNTER_A,e,REASON_EFFECT,tp,tp,count)
        let h = effect::get_handler(d, a.e).nil();
        duel::raise_event(d, h, EVENT_REMOVE_COUNTER + COUNTER_A as u32, Some(a.e), REASON_EFFECT, a.tp, a.tp, count as u32);
    }
}
