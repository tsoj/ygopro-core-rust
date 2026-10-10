//! Continuous Destruction Punch
use super::prelude::*;

pub const ID: u32 = 68057622;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // destroy
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_DAMAGE_STEP_END);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_condition(d, e2, Some(condition(descon)));
    effect::set_operation(d, e2, Some(operation(desop)));
    card::register_effect(d, c, e2, false);
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let attacker = duel::get_attacker(d);
    let Some(at) = duel::get_attack_target(d) else {
        return false;
    };
    let att = attacker.nil();
    card::is_controler(d, att, 1 - tp)
        && card::is_relate_to_battle(d, att)
        && card::is_defense_pos(d, at)
        && card::is_relate_to_battle(d, at)
        && card::get_attack(d, att) < card::get_defense(d, at)
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, _a: &EffArgs) {
    let att = duel::get_attacker(d).nil();
    duel::destroy(d, att, REASON_EFFECT, None, None);
}
