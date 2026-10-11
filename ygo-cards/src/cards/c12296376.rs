//! Wattwoodpecker
use super::prelude::*;

pub const ID: u32 = 12296376;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Can make a second attack
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
    effect::set_value(d, e1, 1);
    card::register_effect(d, c, e1, false);
    // Monsters this card battled cannot change their battle positions
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e2, EVENT_BATTLED);
    effect::set_operation(d, e2, Some(operation(op)));
    card::register_effect(d, c, e2, false);
}
fn op(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let bc = match card::get_battle_target(d, c) {
        Some(bc) => bc,
        None => return,
    };
    // Cannot change its battle position
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, 3313);
    effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_CANNOT_CHANGE_POSITION);
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, bc, e1, false);
}
