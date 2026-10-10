//! Megamorph
use super::prelude::*;

pub const ID: u32 = 22046459;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // Atk Change
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e2, EFFECT_SET_ATTACK);
    effect::set_condition(d, e2, Some(condition(cond)));
    effect::set_value_fn(d, e2, effect_card_value(value));
    card::register_effect(d, c, e2, false);
}
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::get_lp(d, 0) != duel::get_lp(d, 1)
}
fn value(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    let p = card::get_controler(d, effect::get_handler(d, e).nil());
    if duel::get_lp(d, p) < duel::get_lp(d, 1 - p) {
        card::get_base_attack(d, c) as i64 * 2
    } else if duel::get_lp(d, p) > duel::get_lp(d, 1 - p) {
        lua_int(card::get_base_attack(d, c) as f64 / 2.0)
    } else {
        // Lua returns nil here, which the core reads as 0
        0
    }
}
