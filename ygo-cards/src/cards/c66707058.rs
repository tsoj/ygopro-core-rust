//! Goblin Black Ops
use super::prelude::*;

pub const ID: u32 = 66707058;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // to defense
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_PHASE | PHASE_BATTLE);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(poscon)));
    effect::set_operation(d, e1, Some(operation(posop)));
    card::register_effect(d, c, e1, false);
    // direct
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_DIRECT_ATTACK);
    card::register_effect(d, c, e2, false);
}
fn poscon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::get_attacked_count(d, c) > 0
}
fn posop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_attack_pos(d, c) {
        duel::change_position(d, c, POS_FACEUP_DEFENSE as u8, None, None, None, false);
    }
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_CANNOT_CHANGE_POSITION);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_COPY_INHERIT, 0);
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 3);
    card::register_effect(d, c, e1, false);
}
