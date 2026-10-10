//! Anti-Spell Fragrance
use super::prelude::*;

pub const ID: u32 = 58921041;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // cannot activate
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e2, EFFECT_CANNOT_ACTIVATE);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_target_range(d, e2, 1, 1);
    effect::set_value_fn(d, e2, lua_fn(aclimit));
    card::register_effect(d, c, e2, false);
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e3, EVENT_SSET);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_operation(d, e3, Some(operation(aclimset)));
    card::register_effect(d, c, e3, false);
}
fn aclimit(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    if !effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) || !effect::is_spell_effect(d, re) {
        return false;
    }
    let c = effect::get_handler(d, re).nil();
    !card::is_location(d, c, LOCATION_SZONE) || card::get_flag_effect(d, c, ID) > 0
}
fn aclimset(d: &mut Duel, a: &EffArgs) {
    let eg = a.eg.clone().nil();
    let _ = group::get_first(d, &eg);
    for tc in group::iter(d, &eg) {
        card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END | RESET_OPPO_TURN, 0, 1, 0, 0);
    }
}
