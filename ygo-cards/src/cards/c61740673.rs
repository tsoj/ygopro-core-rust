//! Imperial Order
use super::prelude::*;

pub const ID: u32 = 61740673;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // disable
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_DISABLE);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_target_range(d, e2, LOCATION_SZONE, LOCATION_SZONE);
    effect::set_target(d, e2, Some(effect_card_fn(distarget)));
    card::register_effect(d, c, e2, false);
    // disable effect
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e3, EVENT_CHAIN_SOLVING);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_operation(d, e3, Some(operation(disoperation)));
    card::register_effect(d, c, e3, false);
    // maintain
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e4, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e4, EVENT_PHASE | PHASE_STANDBY);
    effect::set_range(d, e4, LOCATION_SZONE);
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    effect::set_operation(d, e4, Some(operation(mtop)));
    card::register_effect(d, c, e4, false);
}
fn distarget(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let h = effect::get_handler(d, e).nil();
    c != h && card::is_spell(d, c)
}
fn disoperation(d: &mut Duel, a: &EffArgs) {
    let tl = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION])[0].int();
    if (tl & LOCATION_SZONE as i64) != 0 && effect::is_spell_effect(d, a.re.nil()) {
        duel::negate_effect(d, a.ev as u8);
    }
}
fn mtop(d: &mut Duel, a: &EffArgs) {
    if duel::check_lp_cost(d, a.tp, 700) {
        duel::pay_lp_cost(d, a.tp, 700);
    } else {
        let h = effect::get_handler(d, a.e).nil();
        duel::destroy(d, h, REASON_COST, None, None);
    }
}
