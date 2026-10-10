//! Dark Ruler Ha Des
use super::prelude::*;

pub const ID: u32 = 53982768;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot special summon
    let e1 = effect::create_effect(d, c);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_range(d, e1, LOCATION_GRAVE);
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    card::register_effect(d, c, e1, false);
    // Disable
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_BATTLED);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_operation(d, e2, Some(operation(op)));
    card::register_effect(d, c, e2, false);
}
fn op(d: &mut Duel, a: &EffArgs) {
    let attacker = duel::get_attacker(d);
    let dt = duel::get_attack_target(d);
    let p = card::get_controler(d, effect::get_handler(d, a.e).nil());
    let Some(dt) = dt else { return };
    let mut tc: Option<CardId> = None;
    if card::get_controler(d, attacker.nil()) == p && card::is_race(d, attacker.nil(), RACE_FIEND as u64) && card::is_status(d, dt, STATUS_BATTLE_DESTROYED) {
        tc = Some(dt);
    } else if card::get_controler(d, dt) == p && card::is_race(d, dt, RACE_FIEND as u64) && card::is_status(d, attacker.nil(), STATUS_BATTLE_DESTROYED) {
        tc = attacker;
    }
    let Some(tc) = tc else { return };
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_DISABLE);
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_EXC_GRAVE, 1);
    card::register_effect(d, tc, e1, false);
    let e2 = effect::create_effect(d, h);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD_EXC_GRAVE, 1);
    card::register_effect(d, tc, e2, false);
}
