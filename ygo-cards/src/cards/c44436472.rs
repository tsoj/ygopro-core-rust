//! Double Coston
use super::prelude::*;

pub const ID: u32 = 44436472;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // double tribute
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_DOUBLE_TRIBUTE);
    effect::set_value_fn(d, e1, effect_card_fn(condition));
    card::register_effect(d, c, e1, false);
}
fn condition(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_DARK)
    card::is_attribute(d, c, ATTRIBUTE_DARK)
}
