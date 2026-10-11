//! Hero Barrier
use super::prelude::*;

pub const ID: u32 = 44676200;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ELEMENTAL_HERO],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // negate attack
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_ELEMENTAL_HERO)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_ELEMENTAL_HERO as u16])
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp) and Duel.GetCurrentPhase()<PHASE_MAIN2 and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_MZONE,0,1,nil)
    duel::is_turn_player(d, 1 - a.tp)
        && duel::get_current_phase(d) < PHASE_MAIN2
        && duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
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
        // e1:SetOperation(s.disop)
        effect::set_operation(d, e1, Some(operation(disop)));
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    }
}
// function s.disop(e,tp,eg,ep,ev,re,r,rp)
fn disop(d: &mut Duel, a: &EffArgs) {
    // Duel.NegateAttack()
    duel::negate_attack(d);
    // e:Reset()
    effect::reset(d, a.e);
}
