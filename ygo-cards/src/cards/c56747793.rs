//! United We Stand
use super::prelude::*;

pub const ID: u32 = 56747793;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c)
    aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // Atk,def
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_value_fn(d, e2, effect_value(value));
    card::register_effect(d, c, e2, false);
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    effect::set_value_fn(d, e3, effect_value(value));
    card::register_effect(d, c, e3, false);
}
fn value(d: &mut Duel, e: EffectId) -> i64 {
    let h = effect::get_handler(d, e).nil();
    let p = card::get_controler(d, h);
    let n = duel::get_matching_group_count(d, Some(&|d, c| card::is_faceup(d, c)), p, LOCATION_MZONE, 0, NO_EX);
    n as i64 * 800
}
