//! Mage Power
use super::prelude::*;

pub const ID: u32 = 83746708;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c)
    aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // Atk Change
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_value_fn(d, e2, effect_card_value(value));
    card::register_effect(d, c, e2, false);
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    effect::set_value_fn(d, e3, effect_card_value(value));
    card::register_effect(d, c, e3, false);
}
fn value(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    let p = effect::get_handler_player(d, e);
    duel::get_matching_group_count(d, Some(&card::is_spell_trap), p, LOCATION_ONFIELD, 0, NO_EX) as i64 * 500
}
