//! Chronomaly Moai Carrier
use super::prelude::*;

pub const ID: u32 = 38007744;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    card::register_effect(d, c, e1, false);
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    let Some(c) = a.card(1) else { return true };
    let p = card::get_controler(d, c);
    duel::get_field_group_count(d, p, LOCATION_ONFIELD, 0) == 0
        && duel::get_field_group_count(d, p, 0, LOCATION_ONFIELD) > 0
        && duel::get_location_count(d, p, LOCATION_MZONE as u8, None, None, None).0 > 0
}
