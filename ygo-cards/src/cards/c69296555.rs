//! Array of Revealing Light
use super::prelude::*;

pub const ID: u32 = 69296555;

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
    effect::set_code(d, e2, EFFECT_CANNOT_ATTACK);
    effect::set_range(d, e2, LOCATION_FZONE);
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_target(d, e2, Some(effect_card_fn(atktg)));
    card::register_effect(d, c, e2, false);
    effect::set_label_object(d, e1, Value::from(e2));
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RACE as u64);
    let rc = duel::announce_race(d, a.tp, 1, RACE_ALL);
    let e2 = effect::get_label_object(d, a.e).as_effect().nil();
    effect::set_label(d, e2, &[rc as i64]);
    let c = effect::get_handler(d, a.e).nil();
    card::set_hint(d, c, CHINT_RACE as u8, rc);
    true
}
fn atktg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    card::is_status(d, c, STATUS_SUMMON_TURN + STATUS_FLIP_SUMMON_TURN + STATUS_SPSUMMON_TURN)
        && card::is_race(d, c, effect::get_label(d, e) as u64)
}
