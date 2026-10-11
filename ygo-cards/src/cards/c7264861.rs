//! Infernity Beast
use super::prelude::*;

pub const ID: u32 = 7264861;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // actlimit
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target_range(d, e1, 0, 1);
    effect::set_value_fn(d, e1, lua_fn(aclimit));
    effect::set_condition(d, e1, Some(condition(cond)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let p = effect::get_handler_player(d, a.e);
    duel::get_field_group_count(d, p, LOCATION_HAND, 0) == 0 && duel::get_attacker(d) == effect::get_handler(d, a.e)
}
/// `function s.aclimit(e,re,tp) return re:IsHasType(EFFECT_TYPE_ACTIVATE) end`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
}
