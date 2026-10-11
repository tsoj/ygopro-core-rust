//! Dragon of Pride and Soul
use super::prelude::*;

pub const ID: u32 = 96823189;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Special Summon condition
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    card::register_effect(d, c, e1, false);
    // Special Summon from the hand if your opponent has 25 or more cards in the GY
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
    effect::set_range(d, e2, LOCATION_HAND);
    effect::set_condition(d, e2, Some(lua_fn(spcon)));
    card::register_effect(d, c, e2, false);
    // Gains 2500 ATK/DEF if you have 25 or more cards in the GY
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_condition(d, e3, Some(lua_fn(e3_cond)));
    effect::set_value(d, e3, 2500);
    card::register_effect(d, c, e3, false);
    let e4 = effect::clone(d, e3);
    effect::set_code(d, e4, EFFECT_UPDATE_DEFENSE);
    card::register_effect(d, c, e4, false);
}
fn e3_cond(d: &mut Duel, a: Args) -> bool {
    let p = effect::get_handler_player(d, a.e(0));
    duel::get_matching_group_count(d, None, p, LOCATION_GRAVE, 0, NO_EX) >= 25
}
fn spcon(d: &mut Duel, a: Args) -> bool {
    let Some(c) = a.card(1) else { return true };
    let tp = card::get_controler(d, c);
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::get_matching_group_count(d, None, tp, 0, LOCATION_GRAVE, NO_EX) >= 25
}
