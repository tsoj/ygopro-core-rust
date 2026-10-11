//! Rod of the Mind's Eye
use super::prelude::*;

pub const ID: u32 = 94793422;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c)
    aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // Change damage
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e1, EFFECT_CHANGE_BATTLE_DAMAGE);
    effect::set_value_fn(d, e1, aux::change_battle_damage(1, 1000));
    card::register_effect(d, c, e1, false);
}
