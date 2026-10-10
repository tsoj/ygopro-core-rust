//! Shine Palace
use super::prelude::*;

pub const ID: u32 = 82878489;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    aux::add_equip_procedure(d, c, None, Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_attribute(d, c, ATTRIBUTE_LIGHT))), None, None, None, None, None, None);
    // Atk up
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_value(d, e2, 700);
    card::register_effect(d, c, e2, false);
}
