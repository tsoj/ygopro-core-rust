//! Flash Assailant
use super::prelude::*;

pub const ID: u32 = 96890582;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atkdown
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_value_fn(d, e1, effect_card_value(val));
    card::register_effect(d, c, e1, false);
    // defdown
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    card::register_effect(d, c, e2, false);
}
/// `s.val(e,c)`
fn val(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return Duel.GetFieldGroupCount(c:GetControler(),LOCATION_HAND,0)*(-400)
    let p = card::get_controler(d, c);
    duel::get_field_group_count(d, p, LOCATION_HAND, 0) as i64 * -400
}
