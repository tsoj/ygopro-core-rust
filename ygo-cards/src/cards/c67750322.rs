//! Skull Meister
use super::prelude::*;

pub const ID: u32 = 67750322;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Inactivate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DISABLE);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_condition(d, e1, Some(condition(discon)));
    effect::set_cost(d, e1, Some(cost::self_to_grave()));
    effect::set_target(d, e1, Some(cost_target(distg)));
    effect::set_operation(d, e1, Some(operation(disop)));
    card::register_effect(d, c, e1, false);
    // Double Snare
    let e6 = effect::create_effect(d, c);
    effect::set_type(d, e6, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e6, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e6, LOCATION_HAND);
    effect::set_code(d, e6, 3682106);
    card::register_effect(d, c, e6, false);
}
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    let loc = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION])[0].int();
    a.ep != a.tp && duel::is_chain_disablable(d, a.ev as u8) && loc == LOCATION_GRAVE as i64
}
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let rc = effect::get_handler(d, a.re.nil()).nil();
        return !card::is_status(d, rc, STATUS_DISABLED);
    }
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, 0, 0);
    true
}
fn disop(d: &mut Duel, a: &EffArgs) {
    duel::negate_effect(d, a.ev as u8);
}
