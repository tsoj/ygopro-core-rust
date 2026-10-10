//! Chain Summoning
use super::prelude::*;

pub const ID: u32 = 9952083;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetCurrentChain(true)>1 and Duel.CheckChainUniqueness() and Duel.IsTurnPlayer(tp)
    duel::get_current_chain(d, true) > 1 && duel::check_chain_uniqueness(d) && duel::is_turn_player(d, a.tp)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local ct=0
        let mut ct: i64 = 0;
        // local ce={Duel.IsPlayerAffectedByEffect(tp,EFFECT_SET_SUMMON_COUNT_LIMIT)}
        let ce = duel::is_player_affected_by_effect(d, a.tp, EFFECT_SET_SUMMON_COUNT_LIMIT);
        // for _,te in ipairs(ce) do ct=math.max(ct,te:GetValue()) end
        if let Some(te) = ce {
            ct = ct.max(effect::get_value(d, te).int());
        }
        // return ct<3
        return ct < 3;
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SET_SUMMON_COUNT_LIMIT)
    effect::set_code(d, e1, EFFECT_SET_SUMMON_COUNT_LIMIT);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetValue(3)
    effect::set_value(d, e1, 3);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
