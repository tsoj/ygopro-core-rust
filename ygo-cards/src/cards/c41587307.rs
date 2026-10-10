//! Broken Bamboo Sword
use super::prelude::*;

pub const ID: u32 = 41587307;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c)
    aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // Gains 0 ATK
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e0, EFFECT_UPDATE_ATTACK);
    effect::set_value(d, e0, 0);
    card::register_effect(d, c, e0, false);
}
