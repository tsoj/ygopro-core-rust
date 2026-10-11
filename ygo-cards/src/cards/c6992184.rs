//! Pendulum Encore
use super::prelude::*;

pub const ID: u32 = 6992184;

/// `s.should_check`
const SHOULD_CHECK: u64 = 0;
/// `s.exclude_card`
const EXCLUDE_CARD: u64 = 1;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // pendulum summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END as u32));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.pencon)
    effect::set_condition(d, e1, Some(condition(pencon)));
    // e1:SetCost(s.pencost)
    effect::set_cost(d, e1, Some(cost_target(pencost)));
    // e1:SetTarget(s.pentg)
    effect::set_target(d, e1, Some(cost_target(pentg)));
    // e1:SetOperation(s.penop)
    effect::set_operation(d, e1, Some(operation(penop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, move |d| {
        // s.should_check=false
        set_s_var(d, ID, SHOULD_CHECK, Value::Bool(false));
        // s.exclude_card=nil
        set_s_var(d, ID, EXCLUDE_CARD, Value::Nil);
        // local geff=Effect.CreateEffect(c)
        let geff = effect::create_effect(d, c);
        // geff:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, geff, EFFECT_TYPE_FIELD);
        // geff:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
        effect::set_code(d, geff, EFFECT_CANNOT_SPECIAL_SUMMON);
        // geff:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, geff, EFFECT_FLAG_PLAYER_TARGET, 0);
        // geff:SetTargetRange(1,1)
        effect::set_target_range(d, geff, 1, 1);
        // geff:SetTarget(function(e,c)
        //   if s.should_check or s.exclude_card then
        //     return c==s.exclude_card or not c:IsType(TYPE_PENDULUM)
        //   end
        //   return false
        // end)
        effect::set_target(d, geff, Some(effect_card_fn(geff_target)));
        // Duel.RegisterEffect(geff,0)
        duel::register_effect(d, geff, 0);
    });
}
fn geff_target(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    let should_check = s_var(d, ID, SHOULD_CHECK);
    let exclude = s_var(d, ID, EXCLUDE_CARD);
    if should_check.truthy() || exclude.truthy() {
        return exclude.as_card() == Some(c) || !card::is_type(d, c, TYPE_PENDULUM);
    }
    false
}
fn pencon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp) and Duel.IsMainPhase()
    duel::is_turn_player(d, 1 - a.tp) && duel::is_main_phase(d, None)
}
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // s.exclude_card=c
    set_s_var(d, ID, EXCLUDE_CARD, Value::Card(c));
    // local res=c:IsDiscardable() and Duel.IsPlayerCanPendulumSummon(tp)
    let res = card::is_discardable(d, c, None, None) && duel::is_player_can_pendulum_summon(d, tp);
    // s.exclude_card=nil
    set_s_var(d, ID, EXCLUDE_CARD, Value::Nil);
    // return res
    res
}
fn pencost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(100)
    effect::set_label(d, a.e, &[100]);
    // return true
    true
}
fn pentg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        // if e:GetLabel()==100 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND,0,1,e:GetHandler(),tp) end
        if effect::get_label(d, a.e) == 100 {
            let h = effect::get_handler(d, a.e).nil();
            return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, tp)), tp, LOCATION_HAND, 0, 1, h.into());
        }
        // s.should_check=true
        set_s_var(d, ID, SHOULD_CHECK, Value::Bool(true));
        // local res=Duel.IsPlayerCanPendulumSummon(tp)
        let res = duel::is_player_can_pendulum_summon(d, tp);
        // s.should_check=nil
        set_s_var(d, ID, SHOULD_CHECK, Value::Nil);
        // return res
        return res;
    }
    // e:SetLabel(0)
    effect::set_label(d, a.e, &[0]);
    // Duel.DiscardHand(tp,s.filter,1,1,REASON_COST|REASON_DISCARD,nil,tp)
    duel::discard_hand(d, tp, Some(&|d, c| filter(d, c, tp)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA|LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_EXTRA | LOCATION_HAND) as i32);
    true
}
fn addreset(d: &mut Duel, c: CardId) {
    // s.should_check=true
    set_s_var(d, ID, SHOULD_CHECK, Value::Bool(true));
    // local eff=Effect.CreateEffect(c)
    let eff = effect::create_effect(d, c);
    // eff:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, eff, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // eff:SetCode(EVENT_CHAIN_END)
    effect::set_code(d, eff, EVENT_CHAIN_END);
    // eff:SetOperation(function(e) e:Reset() s.should_check=false end)
    effect::set_operation(d, eff, Some(lua_fn(|d, a: Args| -> Value {
        effect::reset(d, a.e(0));
        set_s_var(d, ID, SHOULD_CHECK, Value::Bool(false));
        Value::Nil
    })));
    // Duel.RegisterEffect(eff,0)
    duel::register_effect(d, eff, 0);
}
fn penop(d: &mut Duel, a: &EffArgs) {
    // Duel.PendulumSummon(tp)
    duel::pendulum_summon(d, a.tp);
    // if e:IsHasType(EFFECT_TYPE_ACTIVATE) then
    if effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        let h = effect::get_handler(d, a.e).nil();
        // s.addreset(e:GetHandler())
        addreset(d, h);
        // --indes and can't activate
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
        effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
        // e1:SetTargetRange(LOCATION_PZONE,0)
        effect::set_target_range(d, e1, LOCATION_PZONE, 0);
        // e1:SetValue(aux.indsval)
        effect::set_value_fn(d, e1, aux::indsval());
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_CANNOT_ACTIVATE)
        effect::set_code(d, e2, EFFECT_CANNOT_ACTIVATE);
        // e2:SetValue(1)
        effect::set_value(d, e2, 1);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, a.tp);
        // --return to deck
        // local e3=Effect.CreateEffect(e:GetHandler())
        let e3 = effect::create_effect(d, h);
        // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e3:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
        // e3:SetCountLimit(1)
        effect::set_count_limit(d, e3, 1, 0, 0, 0);
        // e3:SetOperation(s.retop)
        effect::set_operation(d, e3, Some(operation(retop)));
        // e3:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e3,tp)
        duel::register_effect(d, e3, a.tp);
    }
}
fn retop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetFieldGroup(tp,LOCATION_PZONE,0)
    let g = duel::get_field_group(d, a.tp, LOCATION_PZONE, 0);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
