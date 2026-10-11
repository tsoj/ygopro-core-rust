//! Terra Firma Gravity
//! アース・グラビティ
use super::prelude::*;

pub const ID: u32 = 26509612;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[74711057], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_BATTLE_START)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_BATTLE_START));
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
    // return Duel.IsTurnPlayer(1-tp) and Duel.IsBattlePhase()
    duel::is_turn_player(d, 1 - a.tp) && duel::is_battle_phase(d, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,74711057),tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsLevelBelow,4),tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(
            d,
            Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[74711057])),
            a.tp,
            LOCATION_MZONE,
            0,
            1,
            NO_EX,
        ) && duel::is_existing_matching_card(
            d,
            Some(&|d, c| card::is_faceup(d, c) && card::is_level_below(d, c, 4)),
            a.tp,
            0,
            LOCATION_MZONE,
            1,
            NO_EX,
        );
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
    // e1:SetCode(EFFECT_MUST_ATTACK)
    effect::set_code(d, e1, EFFECT_MUST_ATTACK);
    // e1:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    // e1:SetTarget(s.attg)
    effect::set_target(d, e1, Some(effect_card_fn(attg)));
    // e1:SetReset(RESET_PHASE|PHASE_BATTLE)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_BATTLE, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_MUST_ATTACK_MONSTER)
    effect::set_code(d, e2, EFFECT_MUST_ATTACK_MONSTER);
    // e2:SetValue(s.attg2)
    effect::set_value_fn(d, e2, effect_card_fn(attg2));
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
}
/// `s.attg(e,c)`
fn attg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsLevelBelow(4)
    card::is_level_below(d, c, 4)
}
/// `s.attg2(e,c)`
fn attg2(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsCode(74711057)
    card::is_code(d, c, &[74711057])
}
