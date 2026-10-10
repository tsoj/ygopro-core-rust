//! `Gemini.*` (`proc_gemini.lua`): Gemini monsters.

use crate::constants::*;
use crate::lua::{card, effect};
use crate::prelude::*;

/// `Gemini.AddProcedure(c)`: can be Normal Summoned again, and is a Normal Monster until then
pub fn add_procedure(d: &mut Duel, c: CardId) {
    // Can be Normal Summoned again
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_GEMINI_SUMMONABLE);
    card::register_effect(d, c, e1, false);
    // Becomes Normal Type if not yet summoned twice
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_code(d, e2, EFFECT_ADD_TYPE);
    effect::set_range(d, e2, LOCATION_MZONE | LOCATION_GRAVE);
    effect::set_condition(d, e2, Some(normal_status_condition()));
    effect::set_value(d, e2, TYPE_NORMAL as i64);
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EFFECT_REMOVE_TYPE);
    effect::set_value(d, e3, TYPE_EFFECT as i64);
    card::register_effect(d, c, e3, false);
}
/// `Gemini.EffectStatusCondition`: the Gemini monster has its effects
pub fn effect_status_condition() -> LuaFn {
    lua_fn(|d, a| {
        let c = effect::get_handler(d, a.e(0)).nil();
        !card::is_disabled(d, c) && card::is_gemini_state(d, c)
    })
}
/// `Gemini.NormalStatusCondition`
pub fn normal_status_condition() -> LuaFn {
    lua_fn(|d, a| {
        let c = effect::get_handler(d, a.e(0)).nil();
        card::is_faceup(d, c) && !card::is_gemini_state(d, c)
    })
}
