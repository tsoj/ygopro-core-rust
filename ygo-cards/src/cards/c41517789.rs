//! Star Eater
use super::prelude::*;

pub const ID: u32 = 41517789;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro summon
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    card::enable_revive_limit(d, c);
    // spsummon
    let e1 = effect::create_effect(d, c);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    effect::set_value_fn(d, e1, aux::synlimit());
    card::register_effect(d, c, e1, false);
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_CANNOT_DISABLE_SPSUMMON);
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    card::register_effect(d, c, e2, false);
    // summon success
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    effect::set_operation(d, e3, Some(operation(sumsuc)));
    card::register_effect(d, c, e3, false);
    // immune
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e4, EVENT_ATTACK_ANNOUNCE);
    effect::set_operation(d, e4, Some(operation(atkop)));
    card::register_effect(d, c, e4, false);
}
/// `s.sumsuc(e,tp,eg,ep,ev,re,r,rp)`
fn sumsuc(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsSynchroSummoned() then return end
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_synchro_summoned(d, c) {
        return;
    }
    // Duel.SetChainLimitTillChainEnd(aux.FALSE)
    duel::set_chain_limit_till_chain_end(d, aux_false());
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    effect::set_value_fn(d, e1, lua_fn(efilter));
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 1);
    // e:GetHandler():RegisterEffect(e1)
    let h2 = effect::get_handler(d, a.e).nil();
    card::register_effect(d, h2, e1, false);
    // Duel.AdjustInstantly(e:GetHandler())
    let h3 = effect::get_handler(d, a.e).nil();
    duel::adjust_instantly(d, Some(h3));
}
/// `s.efilter(e,te)`: `return te:GetOwner()~=e:GetOwner()`
fn efilter(d: &mut Duel, a: Args<'_>) -> bool {
    effect::get_owner(d, a.e(1)) != effect::get_owner(d, a.e(0))
}
