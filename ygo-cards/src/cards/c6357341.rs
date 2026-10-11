//! The Six Shinobi
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 6357341;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SIX_SAMURAI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Skip the opponent's next turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
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
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsSetCard,SET_SIX_SAMURAI),tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_SIX_SAMURAI as u16]))), a.tp, LOCATION_MZONE, 0, NO_EX);
    // return g:GetClassCount(Card.GetAttribute)==6
    group::get_class_count(d, &g, &|d, c| card::get_attribute(d, c) as i64) == 6
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return not Duel.IsPlayerAffectedByEffect(1-tp,EFFECT_SKIP_TURN) end
    if a.chk == Some(0) {
        return duel::is_player_affected_by_effect(d, 1 - a.tp, EFFECT_SKIP_TURN).is_none();
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
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EFFECT_SKIP_TURN)
    effect::set_code(d, e1, EFFECT_SKIP_TURN);
    // e1:SetTargetRange(0,1)
    effect::set_target_range(d, e1, 0, 1);
    // e1:SetReset(RESET_PHASE|PHASE_END|RESET_OPPO_TURN,Duel.IsTurnPlayer(tp) and 1 or 2)
    let count = if duel::is_turn_player(d, a.tp) { 1 } else { 2 };
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, count);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
