//! DNA Surgery
use super::prelude::*;

pub const ID: u32 = 74701381;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    card::register_effect(d, c, e1, false);
    // race
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_code(d, e2, EFFECT_CHANGE_RACE);
    effect::set_value_fn(d, e2, effect_value(value));
    card::register_effect(d, c, e2, false);
    effect::set_label_object(d, e1, Value::Effect(e2));
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RACE as u64);
    let rc = duel::announce_race(d, a.tp, 1, RACE_ALL);
    if let Some(le) = effect::get_label_object(d, a.e).as_effect() {
        effect::set_label(d, le, &[rc as i64]);
    }
    let c = effect::get_handler(d, a.e).nil();
    card::set_hint(d, c, CHINT_RACE as u8, rc);
    true
}
fn value(d: &mut Duel, e: EffectId) -> i64 {
    effect::get_label(d, e)
}
