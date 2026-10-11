//! Ghoti Fury
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 65898344;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E|TIMING_MAIN_END)
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER_E | TIMING_MAIN_END));
    // e2:SetTarget(s.rmtg)
    effect::set_target(d, e2, Some(cost_target(rmtg)));
    // e2:SetOperation(s.rmop)
    effect::set_operation(d, e2, Some(operation(rmop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e3, CATEGORY_ATKCHANGE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetRange(LOCATION_GRAVE|LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_GRAVE | LOCATION_SZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(s.atkcon)
    effect::set_condition(d, e3, Some(condition(atkcon)));
    // e3:SetCost(s.atkcost)
    effect::set_cost(d, e3, Some(cost_target(atkcost)));
    // e3:SetTarget(s.atktg)
    effect::set_target(d, e3, Some(cost_target(atktg)));
    // e3:SetOperation(s.atkop)
    effect::set_operation(d, e3, Some(operation(atkop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.rmfilter(c,e,tp)
fn rmfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeEffectTarget(e) and c:IsAbleToRemove()
    //   and (c:IsControler(1-tp) or (c:IsFaceup() and c:IsRace(RACE_FISH)))
    card::is_can_be_effect_target(d, c, Some(e))
        && card::is_able_to_remove(d, c, None, None, None)
        && (card::is_controler(d, c, 1 - tp) || (card::is_faceup(d, c) && card::is_race(d, c, RACE_FISH)))
}
// function s.rmrescon(sg,e,tp,mg)
fn rmrescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:FilterCount(Card.IsControler,nil,tp)==1
    (group::filter_count(d, sg, &|d, c| card::is_controler(d, c, tp), NO_EX) == 1, false)
}
// function s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetMatchingGroup(s.rmfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| rmfilter(d, c, a.e, a.tp)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if chk==0 then return aux.SelectUnselectGroup(g,e,tp,2,2,s.rmrescon,0) end
    if a.chk == Some(0) {
        return matches!(
            aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&rmrescon), 0, a.tp, None, None, None, false),
            Err(true)
        );
    }
    // local dg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rmrescon,1,tp,HINTMSG_REMOVE)
    let dg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&rmrescon), 1, a.tp, Some(HINTMSG_REMOVE as u64), None, None, false)
        .unwrap_or_default();
    // Duel.SetTargetCard(dg)
    let n = dg.len() as u8;
    duel::set_target_card(d, dg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,dg,#dg,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(dg), n, 0, 0);
    true
}
// function s.rmop(e,tp,eg,ep,ev,re,r,rp)
fn rmop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg~=2 then return end
    if tg.len() != 2 {
        return;
    }
    // local reset_count=(Duel.IsTurnPlayer(tp) and Duel.IsPhase(PHASE_STANDBY)) and 2 or 1
    let reset_count: u16 = if duel::is_turn_player(d, tp) && duel::is_phase(d, PHASE_STANDBY) { 2 } else { 1 };
    // local turn_chk=Duel.GetTurnCount()
    let turn_chk = duel::get_turn_count(d, None);
    // aux.RemoveUntil(tg,nil,REASON_EFFECT,PHASE_STANDBY,id,e,tp,aux.DefaultFieldReturnOp,
    //   function() return Duel.IsTurnPlayer(tp) and (reset_count==1 or Duel.GetTurnCount()~=turn_chk) end,
    //   RESET_PHASE|PHASE_STANDBY|RESET_SELF_TURN,reset_count)
    let cond: aux::DelayedFn = Arc::new(move |d: &mut Duel, _g: &CardSet, _a: &EffArgs| {
        duel::is_turn_player(d, tp) && (reset_count == 1 || duel::get_turn_count(d, None) != turn_chk)
    });
    aux::remove_until(
        d,
        tg,
        None,
        REASON_EFFECT,
        PHASE_STANDBY,
        ID,
        a.e,
        tp,
        Some(aux::default_field_return()),
        Some(cond),
        Some(RESET_PHASE | PHASE_STANDBY | RESET_SELF_TURN),
        Some(reset_count),
        None,
        None,
    );
}
// function s.atkfilter(c,tp)
fn atkfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_FISH) and c:IsControler(tp)
    card::is_faceup(d, c) && card::is_race(d, c, RACE_FISH) && card::is_controler(d, c, tp)
}
// function s.atkcon(e,tp,eg,ep,ev,re,r,rp)
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.atkfilter,1,nil,tp) and Duel.GetFieldGroupCount(0,LOCATION_REMOVED,LOCATION_REMOVED)>0
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| atkfilter(d, c, a.tp), 1, NO_EX)
        && duel::get_field_group_count(d, 0, LOCATION_REMOVED, LOCATION_REMOVED) > 0
}
// function s.atkcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToRemoveAsCost() and (c:IsLocation(LOCATION_GRAVE) or c:IsStatus(STATUS_EFFECT_ENABLED)) end
    if a.chk == Some(0) {
        return card::is_able_to_remove_as_cost(d, c, None)
            && (card::is_location(d, c, LOCATION_GRAVE) || card::is_status(d, c, STATUS_EFFECT_ENABLED));
    }
    // Duel.Remove(c,POS_FACEUP,REASON_COST)
    duel::remove(d, c, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
// function s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.atkfilter,tp,LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| atkfilter(d, c, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    true
}
// function s.atkop(e,tp,eg,ep,ev,re,r,rp)
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.atkfilter,tp,LOCATION_MZONE,0,nil,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| atkfilter(d, c, a.tp)), a.tp, LOCATION_MZONE, 0, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local ct=Duel.GetFieldGroupCount(0,LOCATION_REMOVED,LOCATION_REMOVED)
    let ct = duel::get_field_group_count(d, 0, LOCATION_REMOVED, LOCATION_REMOVED);
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // local atk=ct*100
    let atk = ct as i64 * 100;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(atk)
        effect::set_value(d, e1, atk);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
