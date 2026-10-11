//! Soul Pendulum
use super::prelude::*;

pub const ID: u32 = 34884015;

/// `local COUNTER_SOUL_PENDULUM=0x200`
const COUNTER_SOUL_PENDULUM: u16 = 0x200;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(COUNTER_SOUL_PENDULUM)
    card::enable_counter_permit(d, c, COUNTER_SOUL_PENDULUM, None, None);
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Change the Pendulum Scale of each card in your Pendulum Zones by 1 (min. 1)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.pendscaletg)
    effect::set_target(d, e1, Some(cost_target(pendscaletg)));
    // e1:SetOperation(s.pendscaleop)
    effect::set_operation(d, e1, Some(operation(pendscaleop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Each time your Pendulum Monster(s) is Pendulum Summoned, place 1 counter on this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(s.countercon)
    effect::set_condition(d, e2, Some(condition(countercon)));
    // e2:SetOperation(function(e) e:GetHandler():AddCounter(COUNTER_SOUL_PENDULUM,1) end)
    effect::set_operation(d, e2, Some(operation(add_counter_op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Pendulum Monsters on the field gain 300 ATK for each counter on this card
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
    // e3:SetTarget(function(e,c) return c:IsType(TYPE_PENDULUM) end)
    effect::set_target(d, e3, Some(effect_card_fn(|d, _e, c| card::is_type(d, c, TYPE_PENDULUM))));
    // e3:SetValue(function(e,c) return e:GetHandler():GetCounter(COUNTER_SOUL_PENDULUM)*300 end)
    effect::set_value_fn(d, e3, effect_card_value(|d, e, _c| {
        let h = effect::get_handler(d, e).nil();
        card::get_counter(d, h, COUNTER_SOUL_PENDULUM) as i64 * 300
    }));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // During your Main Phase this turn, you can conduct 1 Pendulum Summon of a monster(s) in addition to your Pendulum Summon
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCondition(function(e,tp) return Pendulum.PlayerCanGainAdditionalPendulumSummon(tp,id) end)
    effect::set_condition(d, e4, Some(condition(extra_pend_cond)));
    // e4:SetCost(s.extrapendcost)
    effect::set_cost(d, e4, Some(cost_target(extrapendcost)));
    // e4:SetOperation(function(e,tp) Pendulum.GrantAdditionalPendulumSummon(e:GetHandler(),nil,tp,LOCATION_HAND|LOCATION_EXTRA,aux.Stringid(id,2),aux.Stringid(id,3),id) end)
    effect::set_operation(d, e4, Some(operation(extra_pend_op)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.pendscaletg(e,tp,...,chk,chkc)`
fn pendscaletg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,LOCATION_PZONE,0,2,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, a.tp, LOCATION_PZONE, 0, 2, NO_EX);
    }
    // Duel.SetTargetCard(Duel.GetFieldGroup(tp,LOCATION_PZONE,0))
    let g = duel::get_field_group(d, a.tp, LOCATION_PZONE, 0);
    duel::set_target_card(d, g);
    true
}
/// `s.pendscaleop(e,tp,...)`
fn pendscaleop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg==0 then return end
    if tg.is_empty() {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // for sc in tg:Iter() do
    for sc in group::iter(d, &tg) {
        // Duel.HintSelection(sc)
        duel::hint_selection(d, sc, true);
        // local b1=true
        let b1 = true;
        // local b2=sc:GetScale()>1
        let b2 = card::get_scale(d, sc) > 1;
        // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,4)},{b2,aux.Stringid(id,5)})
        let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 4)), (b2, aux::stringid(ID, 5))]);
        // local scale=op==1 and 1 or -1
        let scale: i64 = if op == Some(1) { 1 } else { -1 };
        // Change each target's Pendulum Scale by 1 (min. 1)
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_UPDATE_LSCALE)
        effect::set_code(d, e1, EFFECT_UPDATE_LSCALE);
        // e1:SetValue(scale)
        effect::set_value(d, e1, scale);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // sc:RegisterEffect(e1)
        card::register_effect(d, sc, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_UPDATE_RSCALE)
        effect::set_code(d, e2, EFFECT_UPDATE_RSCALE);
        // sc:RegisterEffect(e2)
        card::register_effect(d, sc, e2, false);
    }
}
/// `s.counterconfilter(c,tp)`
fn counterconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsType(TYPE_PENDULUM) and c:IsPendulumSummoned() and c:IsSummonPlayer(tp)
    card::is_type(d, c, TYPE_PENDULUM) && card::is_pendulum_summoned(d, c) && card::is_summon_player(d, c, tp)
}
/// `s.countercon(e,tp,eg,...)`
fn countercon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.counterconfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| counterconfilter(d, c, tp), 1, NO_EX)
}
/// `function(e) e:GetHandler():AddCounter(COUNTER_SOUL_PENDULUM,1) end`
fn add_counter_op(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    card::add_counter(d, h, COUNTER_SOUL_PENDULUM, 1, false);
}
/// `function(e,tp) return Pendulum.PlayerCanGainAdditionalPendulumSummon(tp,id) end`
fn extra_pend_cond(d: &mut Duel, a: &EffArgs) -> bool {
    pendulum::player_can_gain_additional_pendulum_summon(d, a.tp, ID)
}
/// `s.extrapendcost(e,tp,...,chk)`
fn extrapendcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsCanRemoveCounter(tp,COUNTER_SOUL_PENDULUM,3,REASON_COST) end
    if a.chk == Some(0) {
        return card::is_can_remove_counter(d, c, a.tp, COUNTER_SOUL_PENDULUM, 3, REASON_COST);
    }
    // c:RemoveCounter(tp,COUNTER_SOUL_PENDULUM,3,REASON_COST)
    card::remove_counter(d, c, a.tp, COUNTER_SOUL_PENDULUM, 3, REASON_COST);
    true
}
/// `function(e,tp) Pendulum.GrantAdditionalPendulumSummon(e:GetHandler(),nil,tp,LOCATION_HAND|LOCATION_EXTRA,aux.Stringid(id,2),aux.Stringid(id,3),id) end`
fn extra_pend_op(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    pendulum::grant_additional_pendulum_summon(
        d,
        h,
        None,
        a.tp,
        (LOCATION_HAND | LOCATION_EXTRA) as u32,
        Some(aux::stringid(ID, 2)),
        Some(aux::stringid(ID, 3)),
        ID,
        None,
        None,
    );
}
