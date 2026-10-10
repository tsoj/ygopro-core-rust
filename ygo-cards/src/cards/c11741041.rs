//! Raigeki Bottle
use super::prelude::*;

pub const ID: u32 = 11741041;

pub const INFO: ScriptInfo = ScriptInfo {
    counter_place_list: &[0xc],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(0xc)
    card::enable_counter_permit(d, c, 0xc, None, None);
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // counter
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_operation(d, e2, Some(operation(ctop)));
    card::register_effect(d, c, e2, false);
    // destroy
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_DESTROY);
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_condition(d, e3, Some(condition(descon)));
    effect::set_cost(d, e3, Some(cost::self_to_grave()));
    effect::set_target(d, e3, Some(cost_target(destg)));
    effect::set_operation(d, e3, Some(operation(desop)));
    card::register_effect(d, c, e3, false);
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    let atk = duel::get_attacker(d).nil();
    if card::is_controler(d, atk, a.tp) {
        let c = effect::get_handler(d, a.e).nil();
        card::add_counter(d, c, 0xc, 1, false);
    }
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::get_counter(d, c, 0xc) >= 4
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, None, tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    let g = duel::get_matching_group(d, None, tp, 0, LOCATION_MZONE, NO_EX);
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_matching_group(d, None, a.tp, 0, LOCATION_MZONE, NO_EX);
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
