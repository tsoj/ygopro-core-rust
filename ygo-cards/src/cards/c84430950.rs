//! Armed Samurai - Ben Kei
use super::prelude::*;

pub const ID: u32 = 84430950;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // multi attack
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
    effect::set_value_fn(d, e1, effect_card_value(val));
    card::register_effect(d, c, e1, false);
}
/// `s.val(e,c)`
fn val(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return c:GetEquipCount()
    card::get_equip_count(d, c) as i64
}
