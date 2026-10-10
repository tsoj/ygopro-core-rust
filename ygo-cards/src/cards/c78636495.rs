//! Slate Warrior
use super::prelude::*;

pub const ID: u32 = 78636495;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // flip
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP);
    effect::set_operation(d, e1, Some(operation(operation_flip)));
    card::register_effect(d, c, e1, false);
    // destroyed
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYED);
    effect::set_operation(d, e2, Some(operation(desop)));
    card::register_effect(d, c, e2, false);
}
fn operation_flip(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    effect::set_value(d, e1, 500);
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    card::register_effect(d, c, e2, false);
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let mut tc = duel::get_attacker(d);
    if tc == Some(c) {
        tc = duel::get_attack_target(d);
    }
    let tc = tc.nil();
    if !card::is_relate_to_battle(d, tc) {
        return;
    }
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    effect::set_value(d, e1, -500);
    card::register_effect(d, tc, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    card::register_effect(d, tc, e2, false);
}
