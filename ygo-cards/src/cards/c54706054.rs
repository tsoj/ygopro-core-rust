//! The Calibrator
use super::prelude::*;

pub const ID: u32 = 54706054;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // attack
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_value_fn(d, e1, effect_card_value(atkval));
    card::register_effect(d, c, e1, false);
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    let p = card::get_controler(d, c);
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x)), p, 0, LOCATION_MZONE, NO_EX);
    group::get_sum(d, &g, &|d, x| card::get_rank(d, x) as i64) * 300
}
