//! Solomon's Lawbook
use super::prelude::*;

pub const ID: u32 = 23471572;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_code(d, e1, EFFECT_SKIP_SP);
    // if Duel.IsTurnPlayer(tp) and Duel.IsPhase(PHASE_STANDBY) then
    if duel::is_turn_player(d, a.tp) && duel::is_phase(d, PHASE_STANDBY) {
        // e1:SetCondition(s.skipcon)
        effect::set_condition(d, e1, Some(lua_fn(skipcon)));
        // e1:SetLabel(Duel.GetTurnCount())
        let turn = duel::get_turn_count(d, None) as i64;
        effect::set_label(d, e1, &[turn]);
        // e1:SetReset(RESET_PHASE|PHASE_STANDBY|RESET_SELF_TURN,2)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_STANDBY | RESET_SELF_TURN, 2);
    } else {
        // e1:SetReset(RESET_PHASE|PHASE_STANDBY|RESET_SELF_TURN)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_STANDBY | RESET_SELF_TURN, 1);
    }
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
// function s.skipcon(e)
fn skipcon(d: &mut Duel, a: Args) -> bool {
    // return Duel.GetTurnCount()~=e:GetLabel()
    duel::get_turn_count(d, None) as i64 != effect::get_label(d, a.e(0))
}
