//! Frozen Soul
use super::prelude::*;

pub const ID: u32 = 57069605;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END as u32));
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,...)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetLP(tp)+2000<=Duel.GetLP(1-tp)
    duel::get_lp(d, a.tp) + 2000 <= duel::get_lp(d, 1 - a.tp)
}
/// `s.skipcon(e)`
fn skipcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetTurnCount()~=e:GetLabel()
    duel::get_turn_count(d, None) as i64 != effect::get_label(d, a.e)
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SKIP_BP)
    effect::set_code(d, e1, EFFECT_SKIP_BP);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(0,1)
    effect::set_target_range(d, e1, 0, 1);
    // if Duel.IsTurnPlayer(1-tp) and Duel.IsBattlePhase() then
    if duel::is_turn_player(d, 1 - a.tp) && duel::is_battle_phase(d, None) {
        // e1:SetLabel(Duel.GetTurnCount())
        let tc = duel::get_turn_count(d, None) as i64;
        effect::set_label(d, e1, &[tc]);
        // e1:SetCondition(s.skipcon)
        effect::set_condition(d, e1, Some(condition(skipcon)));
        // e1:SetReset(RESET_PHASE|PHASE_BATTLE|RESET_OPPO_TURN,2)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_BATTLE | RESET_OPPO_TURN, 2);
    } else {
        // e1:SetReset(RESET_PHASE|PHASE_BATTLE|RESET_OPPO_TURN,1)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_BATTLE | RESET_OPPO_TURN, 1);
    }
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
