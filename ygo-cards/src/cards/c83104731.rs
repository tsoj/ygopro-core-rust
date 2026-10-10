//! Ancient Gear Golem
use super::prelude::*;

pub const ID: u32 = 83104731;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot special summon
    let e1 = effect::create_effect(d, c);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    card::register_effect(d, c, e1, false);
    // pierce
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_PIERCE);
    card::register_effect(d, c, e2, false);
    // actlimit
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e3, EFFECT_CANNOT_ACTIVATE);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_target_range(d, e3, 0, 1);
    effect::set_value_fn(d, e3, lua_fn(aclimit));
    effect::set_condition(d, e3, Some(condition(actcon)));
    card::register_effect(d, c, e3, false);
}
/// `s.aclimit(e,re,tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
}
fn actcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_attacker(d) == Some(effect::get_handler(d, a.e).nil())
}
