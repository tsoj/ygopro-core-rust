//! Gren Maju Da Eiza
use super::prelude::*;

pub const ID: u32 = 36584821;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EFFECT_SET_ATTACK);
    effect::set_value_fn(d, e1, effect_card_value(value));
    card::register_effect(d, c, e1, false);

    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_code(d, e2, EFFECT_SET_DEFENSE);
    effect::set_value_fn(d, e2, effect_card_value(value));
    card::register_effect(d, c, e2, false);
}
fn value(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    let p = card::get_controler(d, c);
    duel::get_field_group_count(d, p, LOCATION_REMOVED, 0) as i64 * 400
}
