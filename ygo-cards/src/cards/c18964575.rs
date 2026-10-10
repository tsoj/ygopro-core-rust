//! Swift Scarecrow
use super::prelude::*;

pub const ID: u32 = 18964575;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // end battle phase
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_cost(d, e1, Some(cost::self_discard()));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local at=Duel.GetAttacker()
    let at = duel::get_attacker(d).nil();
    card::get_controler(d, at) != a.tp && duel::get_attack_target(d).is_none()
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    if duel::negate_attack(d) {
        duel::skip_phase(d, 1 - a.tp, PHASE_BATTLE, RESET_PHASE | PHASE_BATTLE_STEP, 1, 0);
    }
}
