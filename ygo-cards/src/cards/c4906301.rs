//! Necro Gardna
use super::prelude::*;

pub const ID: u32 = 4906301;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // disable attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_ATTACK)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_ATTACK));
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e1, Some(cost::self_banish()));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp) and (Duel.IsAbleToEnterBP() or Duel.IsBattlePhase())
    duel::is_turn_player(d, 1 - a.tp) && (duel::is_able_to_enter_bp(d) || duel::is_battle_phase(d, None))
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetAttacker() then Duel.NegateAttack()
    if duel::get_attacker(d).is_some() {
        duel::negate_attack(d);
    } else {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
        effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // e1:SetOperation(s.disop)
        effect::set_operation(d, e1, Some(operation(disop)));
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    }
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, _a: &EffArgs) {
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // Duel.NegateAttack()
    duel::negate_attack(d);
}
