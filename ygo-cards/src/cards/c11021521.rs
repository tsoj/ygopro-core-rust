//! Neko Mane King
use super::prelude::*;

pub const ID: u32 = 11021521;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // end turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return Duel.IsTurnPlayer(1-tp) and c:IsPreviousControler(tp)
    //     and c:IsReason(REASON_EFFECT) and c:GetReasonPlayer()==1-tp
    duel::is_turn_player(d, 1 - tp)
        && card::is_previous_controler(d, c, tp)
        && card::is_reason(d, c, REASON_EFFECT)
        && card::get_reason_player(d, c) == 1 - tp
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.SkipPhase(1-tp,PHASE_DRAW,RESET_PHASE|PHASE_END,1)
    duel::skip_phase(d, 1 - tp, PHASE_DRAW, RESET_PHASE | PHASE_END, 1, 0);
    // Duel.SkipPhase(1-tp,PHASE_STANDBY,RESET_PHASE|PHASE_END,1)
    duel::skip_phase(d, 1 - tp, PHASE_STANDBY, RESET_PHASE | PHASE_END, 1, 0);
    // Duel.SkipPhase(1-tp,PHASE_MAIN1,RESET_PHASE|PHASE_END,1)
    duel::skip_phase(d, 1 - tp, PHASE_MAIN1, RESET_PHASE | PHASE_END, 1, 0);
    // Duel.SkipPhase(1-tp,PHASE_BATTLE,RESET_PHASE|PHASE_END,1,1)
    duel::skip_phase(d, 1 - tp, PHASE_BATTLE, RESET_PHASE | PHASE_END, 1, 1);
    // Duel.SkipPhase(1-tp,PHASE_MAIN2,RESET_PHASE|PHASE_END,1)
    duel::skip_phase(d, 1 - tp, PHASE_MAIN2, RESET_PHASE | PHASE_END, 1, 0);
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_BP)
    effect::set_code(d, e1, EFFECT_CANNOT_BP);
    // e1:SetTargetRange(0,1)
    effect::set_target_range(d, e1, 0, 1);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
