//! Armed Protector Dragon
use super::prelude::*;

pub const ID: u32 = 29330706;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    effect::set_target_range(d, e1, LOCATION_SZONE, 0);
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_type(d, c, TYPE_EQUIP))));
    effect::set_value(d, e1, 1);
    card::register_effect(d, c, e1, false);
    // atkup
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_value_fn(d, e2, effect_card_value(val));
    card::register_effect(d, c, e2, false);
}
fn val(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return c:GetEquipCount()*500
    card::get_equip_count(d, c) as i64 * 500
}
