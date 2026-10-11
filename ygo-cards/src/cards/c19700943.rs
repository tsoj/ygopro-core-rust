//! Dododo Bot
use super::prelude::*;

pub const ID: u32 = 19700943;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // summon limit
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_LIMIT_SUMMON_PROC);
    effect::set_condition(d, e1, Some(lua_fn(|d, a| sumcon(d, a))));
    card::register_effect(d, c, e1, false);
    // immune
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    effect::set_operation(d, e2, Some(operation(atkop)));
    card::register_effect(d, c, e2, false);
}
/// `s.sumcon(e,c,minc)`
fn sumcon(_d: &mut Duel, a: Args<'_>) -> bool {
    // if not c then return true end
    // return false
    a.card(1).is_none()
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    effect::set_value_fn(d, e1, lua_fn(|d, a| efilter(d, a)));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE, 1);
    card::register_effect(d, c, e1, false);
}
/// `s.efilter(e,te)`
fn efilter(d: &mut Duel, a: Args<'_>) -> bool {
    // return te:GetOwner()~=e:GetOwner()
    let te = a.e(1);
    let e = a.e(0);
    effect::get_owner(d, te) != effect::get_owner(d, e)
}
