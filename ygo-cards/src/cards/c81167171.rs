//! Hero Spirit
use super::prelude::*;

pub const ID: u32 = 81167171;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, |d| {
        // s[0]=false
        set_s_var(d, ID, 0, Value::Bool(false));
        // s[1]=false
        set_s_var(d, ID, 1, Value::Bool(false));
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_BATTLE_DESTROYED)
        effect::set_code(d, ge1, EVENT_BATTLE_DESTROYED);
        // ge1:SetOperation(s.checkop1)
        effect::set_operation(d, ge1, Some(operation(checkop1)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
        // aux.AddValuesReset(function()
        //     s[0]=false
        //     s[1]=false
        // end)
        aux::add_values_reset(d, reset_values);
    });
}
/// `aux.AddValuesReset(function() s[0]=false s[1]=false end)`
fn reset_values(d: &mut Duel) {
    set_s_var(d, ID, 0, Value::Bool(false));
    set_s_var(d, ID, 1, Value::Bool(false));
}
/// `s.checkop1(e,tp,eg,ep,ev,re,r,rp)`
fn checkop1(d: &mut Duel, a: &EffArgs) {
    // local tc=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let _ = group::get_first(d, &eg);
    // for tc in aux.Next(eg) do
    for tc in group::iter(d, &eg) {
        // if tc:IsPreviousSetCard(SET_ELEMENTAL_HERO) then
        if card::is_previous_set_card(d, tc, &[SET_ELEMENTAL_HERO as u16]) {
            // s[tc:GetPreviousControler()]=true
            let p = card::get_previous_controler(d, tc);
            set_s_var(d, ID, p as u64, Value::Bool(true));
        }
    }
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return s[tp] and Duel.IsBattlePhase()
    s_var(d, ID, a.tp as u64).truthy() && duel::is_battle_phase(d, None)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_NO_BATTLE_DAMAGE)
    effect::set_code(d, e1, EFFECT_NO_BATTLE_DAMAGE);
    // e1:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    // e1:SetReset(RESET_PHASE|PHASE_BATTLE|PHASE_DAMAGE_CAL)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_BATTLE | PHASE_DAMAGE_CAL, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // local e2=Effect.CreateEffect(e:GetHandler())
    let e2 = effect::create_effect(d, h);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // e2:SetReset(RESET_PHASE|PHASE_BATTLE|PHASE_DAMAGE_CAL)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_BATTLE | PHASE_DAMAGE_CAL, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
}
