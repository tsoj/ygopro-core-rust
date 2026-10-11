//! Ritual Buster
use super::prelude::*;

pub const ID: u32 = 54094821;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRitualSummoned()
    card::is_ritual_summoned(d, c)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| cfilter(d, c), 1, NO_EX)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    // e1:SetTargetRange(0,1)
    effect::set_target_range(d, e1, 0, 1);
    // e1:SetValue(s.aclimit)
    effect::set_value_fn(d, e1, lua_fn(aclimit));
    // if Duel.IsTurnPlayer(tp) and Duel.GetCurrentPhase()<=PHASE_STANDBY then
    if duel::is_turn_player(d, tp) && duel::get_current_phase(d) <= PHASE_STANDBY {
        // e1:SetReset(RESET_PHASE|PHASE_STANDBY|RESET_SELF_TURN,2)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_STANDBY | RESET_SELF_TURN, 2);
    } else {
        // e1:SetReset(RESET_PHASE|PHASE_STANDBY|RESET_SELF_TURN)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_STANDBY | RESET_SELF_TURN, 1);
    }
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// `s.aclimit(e,re,tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    // return re:GetHandler():IsSpellTrap()
    let h = effect::get_handler(d, a.e(1)).nil();
    card::is_spell_trap(d, h)
}
