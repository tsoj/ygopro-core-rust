//! Extinction on Schedule
use super::prelude::*;

pub const ID: u32 = 95676943;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Make players send all cards they control to the GY
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END));
    // e1:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e1, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    // e1:SetCost(Cost.PayLP(2000))
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(2000))));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:IsHasType(EFFECT_TYPE_ACTIVATE) end
    if a.chk == Some(0) {
        return effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE);
    }
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // c:SetTurnCounter(0)
    card::set_turn_counter(d, c, 0);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_PHASE|PHASE_BATTLE)
    effect::set_code(d, e1, EVENT_PHASE | PHASE_BATTLE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetOperation(s.tgop)
    effect::set_operation(d, e1, Some(operation(tgop)));
    // e1:SetReset(RESET_PHASE|PHASE_BATTLE,3)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_BATTLE, 3);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
// function s.tgop(e,tp,eg,ep,ev,re,r,rp)
fn tgop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ct=c:GetTurnCounter()
    let ct = card::get_turn_counter(d, c);
    // ct=ct+1
    let ct = ct + 1;
    // c:SetTurnCounter(ct)
    card::set_turn_counter(d, c, ct as u16);
    // if ct==3 then
    if ct == 3 {
        // local g1=Duel.GetFieldGroup(tp,LOCATION_ONFIELD,0)
        let g1 = duel::get_field_group(d, a.tp, LOCATION_ONFIELD, 0);
        // local g2=Duel.GetFieldGroup(tp,0,LOCATION_ONFIELD)
        let g2 = duel::get_field_group(d, a.tp, 0, LOCATION_ONFIELD);
        // Duel.SendtoGrave(g1,REASON_RULE,PLAYER_NONE,tp)
        duel::send_to_grave(d, g1, REASON_RULE, Some(PLAYER_NONE as u8), Some(a.tp));
        // Duel.SendtoGrave(g2,REASON_RULE,PLAYER_NONE,1-tp)
        duel::send_to_grave(d, g2, REASON_RULE, Some(PLAYER_NONE as u8), Some(1 - a.tp));
    }
}
