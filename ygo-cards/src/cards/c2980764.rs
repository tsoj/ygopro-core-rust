//! Consecrated Light
use super::prelude::*;

pub const ID: u32 = 2980764;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // battle
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_AVOID_BATTLE_DAMAGE);
    effect::set_value_fn(d, e1, lua_fn(tglimit_value));
    card::register_effect(d, c, e1, false);
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    effect::set_value_fn(d, e2, lua_fn(tglimit_value));
    card::register_effect(d, c, e2, false);
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_code(d, e3, EFFECT_CANNOT_ATTACK_ANNOUNCE);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_target(d, e3, Some(effect_card_fn(|d, _e, c| card::is_attribute(d, c, ATTRIBUTE_DARK))));
    card::register_effect(d, c, e3, false);
    // disable spsummon
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_code(d, e4, EFFECT_CANNOT_SUMMON);
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e4, 1, 1);
    effect::set_target(d, e4, Some(effect_card_fn(|d, _e, c| card::is_attribute(d, c, ATTRIBUTE_DARK))));
    card::register_effect(d, c, e4, false);
    let e5 = effect::clone(d, e4);
    effect::set_code(d, e5, EFFECT_CANNOT_SPECIAL_SUMMON);
    card::register_effect(d, c, e5, false);
}
// function(e,c) return c and c:IsAttribute(ATTRIBUTE_DARK) end, used as a value
fn tglimit_value(d: &mut Duel, a: Args) -> bool {
    match a.card(1) {
        Some(c) => card::is_attribute(d, c, ATTRIBUTE_DARK),
        None => false,
    }
}
