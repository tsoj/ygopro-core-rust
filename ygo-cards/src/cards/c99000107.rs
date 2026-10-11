//! Gendo the Ascetic Monk
use super::prelude::*;

pub const ID: u32 = 99000107;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Cannot activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e1, 1, 1);
    effect::set_value_fn(d, e1, lua_fn(aclimit));
    card::register_effect(d, c, e1, false);
    //
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    effect::set_condition(d, e2, Some(lua_fn(tgcon)));
    effect::set_value(d, e2, 1);
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_EFFECT);
    effect::set_value(d, e3, 1);
    card::register_effect(d, c, e3, false);
}
/// `function s.aclimit(e,re,tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    let tc = effect::get_handler(d, re).nil();
    card::is_location(d, tc, LOCATION_MZONE) && card::is_faceup(d, tc) && card::is_attack_pos(d, tc) && effect::is_monster_effect(d, re)
}
/// `function s.tgcon(e)`
fn tgcon(d: &mut Duel, a: Args) -> bool {
    let c = effect::get_handler(d, a.e(0)).nil();
    card::is_defense_pos(d, c)
}
