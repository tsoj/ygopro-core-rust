//! Wattmole
use super::prelude::*;

pub const ID: u32 = 32548609;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // extra attack
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
    effect::set_value(d, e1, 1);
    card::register_effect(d, c, e1, false);
    // destroy
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_BATTLE_START);
    effect::set_condition(d, e2, Some(condition(descon)));
    effect::set_target(d, e2, Some(cost_target(destg)));
    effect::set_operation(d, e2, Some(operation(desop)));
    card::register_effect(d, c, e2, false);
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e);
    if h != duel::get_attacker(d) {
        return false;
    }
    match duel::get_attack_target(d) {
        Some(dt) => card::is_facedown(d, dt) && card::is_defense_pos(d, dt),
        None => false,
    }
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let dt = duel::get_attack_target(d).nil();
        return card::is_relate_to_battle(d, dt);
    }
    let dt = duel::get_attack_target(d);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, dt.map(CardSet::from_card), 1, 0, 0);
    true
}
fn desop(d: &mut Duel, _a: &EffArgs) {
    let dt = duel::get_attack_target(d).nil();
    if card::is_relate_to_battle(d, dt) {
        duel::destroy(d, dt, REASON_EFFECT, None, None);
    }
}
