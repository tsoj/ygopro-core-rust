//! The Trojan Horse
use super::prelude::*;

pub const ID: u32 = 38479725;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // double tribute
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_DOUBLE_TRIBUTE);
    effect::set_value_fn(d, e1, lua_fn(cond));
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,c)`
fn cond(d: &mut Duel, a: Args) -> bool {
    // return c:IsAttribute(ATTRIBUTE_EARTH)
    let c = a.c(1);
    card::is_attribute(d, c, ATTRIBUTE_EARTH)
}
