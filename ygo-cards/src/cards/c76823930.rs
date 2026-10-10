//! Ultimeat Offering
use super::prelude::*;

pub const ID: u32 = 76823930;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Allow up to 3 Normal Summons
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(_,tp) return Duel.IsMainPhase() and Duel.IsTurnPlayer(tp) end)
    effect::set_condition(d, e2, Some(condition(cond_e2)));
    // e2:SetCost(s.cost(1000))
    effect::set_cost(d, e2, Some(cost(1000)));
    // e2:SetTarget(s.trisumtg)
    effect::set_target(d, e2, Some(cost_target(trisumtg)));
    // e2:SetOperation(s.trisumop)
    effect::set_operation(d, e2, Some(operation(trisumop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Normal Summon 1 monster during your opponent's Battle Phase
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SUMMON)
    effect::set_category(d, e3, CATEGORY_SUMMON);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetHintTiming(0,TIMING_BATTLE_START)
    effect::set_hint_timing(d, e3, 0, Some(TIMING_BATTLE_START));
    // e3:SetCondition(function(_,tp) return Duel.IsBattlePhase() and Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e3, Some(condition(cond_e3)));
    // e3:SetCost(s.cost(500))
    effect::set_cost(d, e3, Some(cost(500)));
    // e3:SetTarget(s.nstg)
    effect::set_target(d, e3, Some(cost_target(nstg)));
    // e3:SetOperation(s.nsop)
    effect::set_operation(d, e3, Some(operation(nsop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(_,tp) return Duel.IsMainPhase() and Duel.IsTurnPlayer(tp) end`
fn cond_e2(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_main_phase(d, None) && duel::is_turn_player(d, a.tp)
}
/// `function(_,tp) return Duel.IsBattlePhase() and Duel.IsTurnPlayer(1-tp) end`
fn cond_e3(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_battle_phase(d, None) && duel::is_turn_player(d, 1 - a.tp)
}
/// `s.cost(lpcost)`
fn cost(lpcost: u32) -> LuaFn {
    cost_target(move |d, a| {
        if a.chk == Some(0) {
            return duel::check_lp_cost(d, a.tp, lpcost);
        }
        duel::pay_lp_cost(d, a.tp, lpcost);
        true
    })
}
/// `s.trisumtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn trisumtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // local ct=0
        let mut ct: i64 = 0;
        // local ce={Duel.IsPlayerAffectedByEffect(tp,EFFECT_SET_SUMMON_COUNT_LIMIT)}
        // for _,te in ipairs(ce) do ct=math.max(ct,te:GetValue()) end
        if let Some(te) = duel::is_player_affected_by_effect(d, a.tp, EFFECT_SET_SUMMON_COUNT_LIMIT) {
            ct = ct.max(effect::get_value(d, te).int());
        }
        // return ct<3
        return ct < 3;
    }
    true
}
/// `s.trisumop(e,tp,eg,ep,ev,re,r,rp)`
fn trisumop(d: &mut Duel, a: &EffArgs) {
    // Player can Normal Summon/Set up to 3 times
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_SET_SUMMON_COUNT_LIMIT)
    effect::set_code(d, e1, EFFECT_SET_SUMMON_COUNT_LIMIT);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetValue(3)
    effect::set_value(d, e1, 3);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
fn summonable_filter(d: &mut Duel, c: CardId) -> bool {
    card::is_summonable(d, c, true, None, 0, None)
}
/// `s.nstg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn nstg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&summonable_filter), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_MZONE) as i32);
    true
}
/// `s.nsop(e,tp,eg,ep,ev,re,r,rp)`
fn nsop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,Card.IsSummonable,tp,LOCATION_HAND|LOCATION_MZONE,0,1,1,nil,true,nil):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&summonable_filter), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if tc then
    if let Some(tc) = tc {
        // Duel.Summon(tp,tc,true,nil)
        duel::summon(d, a.tp, tc, true, None, 0, None);
    }
}
