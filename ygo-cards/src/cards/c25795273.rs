//! Al-Lumi'raj
use super::prelude::*;

pub const ID: u32 = 25795273;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atk down
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_value_fn(d, e1, effect_card_value(val));
    card::register_effect(d, c, e1, false);
    // def down
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_value_fn(d, e2, effect_card_value(val));
    card::register_effect(d, c, e2, false);
}
fn val(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    if card::is_type(d, c, TYPE_XYZ) {
        card::get_rank(d, c) as i64 * -300
    } else {
        card::get_level(d, c) as i64 * -300
    }
}
