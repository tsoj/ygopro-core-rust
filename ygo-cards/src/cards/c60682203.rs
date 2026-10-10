//! Cold Wave
use super::prelude::*;

pub const ID: u32 = 60682203;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsPhase(PHASE_MAIN1) and not Duel.CheckPhaseActivity()
    duel::is_phase(d, PHASE_MAIN1) && !duel::check_phase_activity(d)
}
fn op(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    effect::set_target_range(d, e1, 1, 1);
    effect::set_value_fn(d, e1, lua_fn(aclimit));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 2);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // cannot set
    // local e2=Effect.CreateEffect(e:GetHandler())
    let e2 = effect::create_effect(d, h);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_CANNOT_SSET);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e2, 1, 1);
    effect::set_target(d, e2, Some(aux_true()));
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 2);
    duel::register_effect(d, e2, a.tp);
    // local e3=Effect.CreateEffect(e:GetHandler())
    let e3 = effect::create_effect(d, h);
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 2);
    effect::set_target_range(d, e3, 1, 1);
    duel::register_effect(d, e3, a.tp);
}
fn aclimit(d: &mut Duel, a: Args) -> bool {
    // return re:IsSpellTrapEffect()
    let re = a.e(1);
    effect::is_spell_trap_effect(d, re)
}
