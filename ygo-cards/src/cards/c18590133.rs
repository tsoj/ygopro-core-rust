//! Goblin King
use super::prelude::*;

pub const ID: u32 = 18590133;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot be battle target
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EFFECT_CANNOT_BE_BATTLE_TARGET);
    effect::set_condition(d, e1, Some(lua_fn(ccon)));
    effect::set_value_fn(d, e1, aux::imval1());
    card::register_effect(d, c, e1, false);
    // atk,def
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    effect::set_code(d, e2, EFFECT_SET_ATTACK);
    effect::set_value_fn(d, e2, effect_card_value(val));
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EFFECT_SET_DEFENSE);
    card::register_effect(d, c, e3, false);
}
fn ccon(d: &mut Duel, a: Args) -> bool {
    let c = effect::get_handler(d, a.e(0)).nil();
    let p = card::get_controler(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| card::is_faceup(d, x) && card::is_race(d, x, RACE_FIEND)), p, LOCATION_MZONE, 0, 1, c.into())
}
fn val(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    let p = card::get_controler(d, c);
    duel::get_matching_group_count(d, Some(&|d, x| card::is_faceup(d, x) && card::is_race(d, x, RACE_FIEND)), p, LOCATION_MZONE, LOCATION_MZONE, c.into()) as i64 * 1000
}
