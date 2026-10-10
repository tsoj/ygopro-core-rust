//! Command Knight
use super::prelude::*;

pub const ID: u32 = 10375182;

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
    // atk
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_race(d, c, RACE_WARRIOR))));
    effect::set_value(d, e2, 400);
    card::register_effect(d, c, e2, false);
}
fn ccon(d: &mut Duel, a: Args) -> bool {
    let p = effect::get_handler_player(d, a.e(0));
    duel::get_field_group_count(d, p, LOCATION_MZONE, 0) > 1
}
