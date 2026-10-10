//! Mirage Dragon
use super::prelude::*;

pub const ID: u32 = 15960641;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // act limit
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    effect::set_condition(d, e1, Some(condition(con)));
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target_range(d, e1, 0, 1);
    effect::set_value_fn(d, e1, lua_fn(aclimit));
    card::register_effect(d, c, e1, false);
}
fn con(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_battle_phase(d, None)
}
/// `function(e,re,tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    let h = effect::get_handler(d, re).nil();
    card::is_trap(d, h) && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
}
