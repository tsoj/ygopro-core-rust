//! Spirit Ryu
use super::prelude::*;

pub const ID: u32 = 67957315;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atk up
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetHintTiming(TIMING_BATTLE_PHASE|TIMING_CHAIN_END)
    effect::set_hint_timing(d, e1, TIMING_BATTLE_PHASE | TIMING_CHAIN_END, None);
    // e1:SetCondition(s.adcon)
    effect::set_condition(d, e1, Some(condition(adcon)));
    // e1:SetCost(s.adcost)
    effect::set_cost(d, e1, Some(cost_target(adcost)));
    // e1:SetOperation(s.adop)
    effect::set_operation(d, e1, Some(operation(adop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.adcon(e,tp,eg,ep,ev,re,r,rp)`
fn adcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp) and Duel.IsBattlePhase()
    //   and (e:GetHandler()==Duel.GetAttacker() or e:GetHandler()==Duel.GetAttackTarget())
    let c = effect::get_handler(d, a.e);
    duel::is_turn_player(d, a.tp) && duel::is_battle_phase(d, None) && (c == duel::get_attacker(d) || c == duel::get_attack_target(d))
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_DRAGON) and c:IsDiscardable() and c:IsAbleToGraveAsCost()
    card::is_race(d, c, RACE_DRAGON) && card::is_discardable(d, c, None, None) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.adcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn adcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.DiscardHand(tp,s.cfilter,1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, tp, Some(&cfilter), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
/// `s.adop(e,tp,eg,ep,ev,re,r,rp)`
fn adop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(1000)
        effect::set_value(d, e1, 1000);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE|RESET_PHASE|PHASE_BATTLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE | RESET_PHASE | PHASE_BATTLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_UPDATE_DEFENSE)
        effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
    }
}
