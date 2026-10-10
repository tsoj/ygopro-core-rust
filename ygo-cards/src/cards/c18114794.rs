//! Summon Breaker
use super::prelude::*;

pub const ID: u32 = 18114794;

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
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e2, EVENT_CUSTOM + ID);
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // aux.GlobalCheck(s,function() ... end)
    aux::global_check(d, ID, |d| {
        // s[0]=0
        // s[1]=0
        set_s_var(d, ID, 0, Value::Int(0));
        set_s_var(d, ID, 1, Value::Int(0));
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_SUMMON_SUCCESS)
        effect::set_code(d, ge1, EVENT_SUMMON_SUCCESS);
        // ge1:SetOperation(s.checkop)
        effect::set_operation(d, ge1, Some(operation(checkop)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
        // local ge2=Effect.CreateEffect(c)
        let ge2 = effect::create_effect(d, c);
        // ge2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge2:SetCode(EVENT_FLIP_SUMMON_SUCCESS)
        effect::set_code(d, ge2, EVENT_FLIP_SUMMON_SUCCESS);
        // ge2:SetOperation(s.checkop)
        effect::set_operation(d, ge2, Some(operation(checkop)));
        // Duel.RegisterEffect(ge2,0)
        duel::register_effect(d, ge2, 0);
        // local ge3=Effect.CreateEffect(c)
        let ge3 = effect::create_effect(d, c);
        // ge3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge3:SetCode(EVENT_SPSUMMON_SUCCESS)
        effect::set_code(d, ge3, EVENT_SPSUMMON_SUCCESS);
        // ge3:SetOperation(s.checkop)
        effect::set_operation(d, ge3, Some(operation(checkop)));
        // Duel.RegisterEffect(ge3,0)
        duel::register_effect(d, ge3, 0);
    });
}
/// `s.checkop(e,tp,eg,ep,ev,re,r,rp)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    // local turnp=Duel.GetTurnPlayer()
    let turnp = duel::get_turn_player(d);
    // if Duel.GetTurnCount()~=s[2] then
    let turn = duel::get_turn_count(d, None) as i64;
    let same = matches!(s_var(d, ID, 2), Value::Int(x) if x == turn);
    if !same {
        // s[0]=0
        // s[1]=0
        // s[2]=Duel.GetTurnCount()
        set_s_var(d, ID, 0, Value::Int(0));
        set_s_var(d, ID, 1, Value::Int(0));
        set_s_var(d, ID, 2, Value::Int(turn));
    }
    // local p1=false
    let mut p1 = false;
    // for tc in aux.Next(eg) do
    let eg = a.eg();
    for tc in group::iter(d, &eg) {
        // if tc:GetSummonPlayer()==turnp then p1=true end
        if card::get_summon_player(d, tc) == turnp {
            p1 = true;
        }
    }
    // if p1 then
    if p1 {
        // s[turnp]=s[turnp]+1
        let cnt = s_var(d, ID, turnp as u64).int() + 1;
        set_s_var(d, ID, turnp as u64, Value::Int(cnt));
        // if s[turnp]==3 then
        if cnt == 3 {
            // Duel.RaiseEvent(e:GetHandler(),EVENT_CUSTOM+id,e,0,0,0,0)
            let h = effect::get_handler(d, a.e).nil();
            duel::raise_event(d, h, EVENT_CUSTOM + ID, Some(a.e), 0, 0, 0, 0);
        }
    }
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsPhase(PHASE_MAIN1)
    duel::is_phase(d, PHASE_MAIN1)
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local turnp=Duel.GetTurnPlayer()
    let turnp = duel::get_turn_player(d);
    // Duel.SkipPhase(turnp,PHASE_MAIN1,RESET_PHASE|PHASE_END,1)
    duel::skip_phase(d, turnp, PHASE_MAIN1, RESET_PHASE | PHASE_END, 1, 0);
    // Duel.SkipPhase(turnp,PHASE_BATTLE,RESET_PHASE|PHASE_END,1,1)
    duel::skip_phase(d, turnp, PHASE_BATTLE, RESET_PHASE | PHASE_END, 1, 1);
    // Duel.SkipPhase(turnp,PHASE_MAIN2,RESET_PHASE|PHASE_END,1)
    duel::skip_phase(d, turnp, PHASE_MAIN2, RESET_PHASE | PHASE_END, 1, 0);
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, h);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_BP)
    effect::set_code(d, e1, EFFECT_CANNOT_BP);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,turnp)
    duel::register_effect(d, e1, turnp);
}
