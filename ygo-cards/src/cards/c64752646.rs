//! Fire Princess
use super::prelude::*;

pub const ID: u32 = 64752646;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // damage
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EVENT_RECOVER);
    effect::set_condition(d, e1, Some(condition(cd)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn cd(_d: &mut Duel, a: &EffArgs) -> bool {
    // return tp==ep
    a.tp == a.ep
}
fn op(d: &mut Duel, a: &EffArgs) {
    // Duel.Damage(1-tp,500,REASON_EFFECT)
    duel::damage(d, 1 - a.tp, 500, REASON_EFFECT, false, None);
}
