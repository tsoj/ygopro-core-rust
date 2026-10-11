//! Supernatural Regeneration
use super::prelude::*;

pub const ID: u32 = 49980185;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // recover
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCategory(CATEGORY_RECOVER)
    effect::set_category(d, e1, CATEGORY_RECOVER);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, TIMING_END_PHASE, None);
    // e1:SetCondition(s.reccon)
    effect::set_condition(d, e1, Some(condition(reccon)));
    // e1:SetTarget(s.rectg)
    effect::set_target(d, e1, Some(cost_target(rectg)));
    // e1:SetOperation(s.recop)
    effect::set_operation(d, e1, Some(operation(recop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.GlobalCheck(s,function() ... end)
    aux::global_check(d, ID, |d: &mut Duel| {
        // s[0]=0
        set_s_var(d, ID, 0, Value::Int(0));
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_TO_GRAVE)
        effect::set_code(d, ge1, EVENT_TO_GRAVE);
        // ge1:SetOperation(s.checkop)
        effect::set_operation(d, ge1, Some(operation(checkop)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
        // aux.AddValuesReset(function() s[0]=0 end)
        aux::add_values_reset(d, |d: &mut Duel| {
            set_s_var(d, ID, 0, Value::Int(0));
        });
    });
}
/// `s.checkop(e,tp,eg,ep,ev,re,r,rp)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    // s[0]=s[0]+eg:FilterCount(Card.IsRace,nil,RACE_PSYCHIC)
    let old = s_var(d, ID, 0).int();
    let eg = a.eg.clone().nil();
    let n = group::filter_count(d, &eg, &|d: &mut Duel, c| card::is_race(d, c, RACE_PSYCHIC), NO_EX);
    set_s_var(d, ID, 0, Value::Int(old + n as i64));
}
/// `s.reccon(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reccon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsPhase(PHASE_END)
    duel::is_phase(d, PHASE_END)
}
/// `s.rectg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rectg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return s[0]~=0 end
    if a.chk == Some(0) {
        return s_var(d, ID, 0).int() != 0;
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(s[0]*1000)
    let amount = s_var(d, ID, 0).int() * 1000;
    duel::set_target_param(d, amount as u32);
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,s[0]*1000)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, amount as i32);
    true
}
/// `s.recop(e,tp,eg,ep,ev,re,r,rp)`
fn recop(d: &mut Duel, _a: &EffArgs) {
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // Duel.Recover(p,s[0]*1000,REASON_EFFECT)
    let amount = s_var(d, ID, 0).int() * 1000;
    duel::recover(d, p, amount, REASON_EFFECT, false, None);
}
