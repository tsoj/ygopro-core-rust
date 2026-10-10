//! Messenger of Peace
use super::prelude::*;

pub const ID: u32 = 44656491;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // cannot attack
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_CANNOT_ATTACK_ANNOUNCE);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_target(d, e2, Some(effect_card_fn(atktarget)));
    card::register_effect(d, c, e2, false);
    // maintain
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e3, EVENT_PHASE | PHASE_STANDBY);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_condition(d, e3, Some(condition(mtcon)));
    effect::set_operation(d, e3, Some(operation(mtop)));
    card::register_effect(d, c, e3, false);
}
/// `s.atktarget(e,c)`: `return c:GetAttack()>=1500`
fn atktarget(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::get_attack(d, c) >= 1500
}
/// `s.mtcon(e,tp,eg,ep,ev,re,r,rp)`
fn mtcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp)
    duel::is_turn_player(d, a.tp)
}
/// `s.mtop(e,tp,eg,ep,ev,re,r,rp)`
fn mtop(d: &mut Duel, a: &EffArgs) {
    // if Duel.CheckLPCost(tp,100) and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if duel::check_lp_cost(d, a.tp, 100) && duel::select_yes_no(d, a.tp, aux::stringid(ID, 0)) {
        // Duel.PayLPCost(tp,100)
        duel::pay_lp_cost(d, a.tp, 100);
    } else {
        // Duel.Destroy(e:GetHandler(),REASON_COST)
        let h = effect::get_handler(d, a.e).nil();
        duel::destroy(d, h, REASON_COST, None, None);
    }
}
