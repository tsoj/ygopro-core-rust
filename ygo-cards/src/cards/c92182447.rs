//! Mischief of the Time Goddess
use super::prelude::*;

pub const ID: u32 = 92182447;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[92182447], listed_series: &[SET_VALKYRIE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMING_BATTLE_END)
    effect::set_hint_timing(d, e1, TIMING_BATTLE_END as u32, None);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // if Duel.IsTurnPlayer(1-tp) or Duel.GetCurrentPhase()~=PHASE_BATTLE or Duel.GetCurrentChain(true)>0 then
    //     return false
    // end
    if duel::is_turn_player(d, 1 - a.tp) || duel::get_current_phase(d) != PHASE_BATTLE || duel::get_current_chain(d, true) > 0 {
        return false;
    }
    // local g=Duel.GetFieldGroup(tp,LOCATION_MZONE,0)
    let g = duel::get_field_group(d, a.tp, LOCATION_MZONE, 0);
    // return #g>0 and g:FilterCount(aux.FaceupFilter(Card.IsSetCard,SET_VALKYRIE),nil)==#g
    !g.is_empty() && group::filter_count(d, &g, &faceup_valkyrie, NO_EX) == g.len()
}
// aux.FaceupFilter(Card.IsSetCard,SET_VALKYRIE)
fn faceup_valkyrie(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_VALKYRIE as u16])
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // if e:IsHasType(EFFECT_TYPE_ACTIVATE) then
    if effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        // Duel.SetChainLimit(aux.FALSE)
        duel::set_chain_limit(d, aux_false());
    }
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then Duel.SendtoGrave(c,REASON_EFFECT) end
    if card::is_relate_to_effect(d, c, a.e) {
        duel::send_to_grave(d, c, REASON_EFFECT, None, None);
    }
    // --skip phases until Battle Phase
    // Duel.SkipPhase(tp,PHASE_BATTLE,RESET_PHASE|PHASE_END,1)
    duel::skip_phase(d, a.tp, PHASE_BATTLE, RESET_PHASE | PHASE_END, 1, 0);
    // --prevent activations for the rest of that battle phase
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD);
    // e0:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e0, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e0:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e0, EFFECT_CANNOT_ACTIVATE);
    // e0:SetTargetRange(0,1)
    effect::set_target_range(d, e0, 0, 1);
    // e0:SetValue(1)
    effect::set_value(d, e0, 1);
    // e0:SetReset(RESET_PHASE|PHASE_BATTLE|PHASE_END)
    effect::set_reset(d, e0, RESET_PHASE | PHASE_BATTLE | PHASE_END, 1);
    // Duel.RegisterEffect(e0,tp)
    duel::register_effect(d, e0, a.tp);
    // Duel.SkipPhase(tp,PHASE_MAIN2,RESET_PHASE|PHASE_END,1)
    duel::skip_phase(d, a.tp, PHASE_MAIN2, RESET_PHASE | PHASE_END, 1, 0);
    // Duel.SkipPhase(tp,PHASE_END,RESET_PHASE|PHASE_END,1)
    duel::skip_phase(d, a.tp, PHASE_END, RESET_PHASE | PHASE_END, 1, 0);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EFFECT_SKIP_TURN)
    effect::set_code(d, e1, EFFECT_SKIP_TURN);
    // e1:SetTargetRange(0,1)
    effect::set_target_range(d, e1, 0, 1);
    // e1:SetReset(RESET_PHASE|PHASE_END|RESET_OPPO_TURN)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // Duel.SkipPhase(tp,PHASE_DRAW,RESET_PHASE|PHASE_END,2)
    duel::skip_phase(d, a.tp, PHASE_DRAW, RESET_PHASE | PHASE_END, 2, 0);
    // Duel.SkipPhase(tp,PHASE_STANDBY,RESET_PHASE|PHASE_END,2)
    duel::skip_phase(d, a.tp, PHASE_STANDBY, RESET_PHASE | PHASE_END, 2, 0);
    // Duel.SkipPhase(tp,PHASE_MAIN1,RESET_PHASE|PHASE_END,2)
    duel::skip_phase(d, a.tp, PHASE_MAIN1, RESET_PHASE | PHASE_END, 2, 0);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EFFECT_CANNOT_EP)
    effect::set_code(d, e2, EFFECT_CANNOT_EP);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // e2:SetReset(RESET_PHASE|PHASE_MAIN1|RESET_SELF_TURN)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_MAIN1 | RESET_SELF_TURN, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
    // --prevent activation of "Mischief of the Time Goddess"
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e3, EFFECT_CANNOT_ACTIVATE);
    // e3:SetTargetRange(1,0)
    effect::set_target_range(d, e3, 1, 0);
    // e3:SetValue(s.limit)
    effect::set_value_fn(d, e3, lua_fn(limit));
    // e3:SetReset(RESET_PHASE|PHASE_END,3)
    effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 3);
    // Duel.RegisterEffect(e3,tp)
    duel::register_effect(d, e3, a.tp);
}
// function s.limit(e,re,tp)
fn limit(d: &mut Duel, a: Args) -> bool {
    // return re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:GetHandler():IsCode(92182447)
    let re = a.e(1);
    if !effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) {
        return false;
    }
    let h = effect::get_handler(d, re).nil();
    card::is_code(d, h, &[92182447])
}
