//! Jinzo
use super::prelude::*;

pub const ID: u32 = 77585513;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot trigger
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CANNOT_TRIGGER);
    effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target_range(d, e1, LOCATION_HAND | LOCATION_SZONE, LOCATION_HAND | LOCATION_SZONE);
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_trap(d, c))));
    card::register_effect(d, c, e1, false);
    // cannot activate
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_CANNOT_ACTIVATE);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e2, 1, 1);
    effect::set_value_fn(d, e2, lua_fn(aclimit));
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    card::register_effect(d, c, e2, false);
    // disable
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_code(d, e3, EFFECT_DISABLE);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_target_range(d, e3, LOCATION_SZONE, LOCATION_SZONE);
    effect::set_target(d, e3, Some(effect_card_fn(|d, _e, c| card::is_trap(d, c))));
    card::register_effect(d, c, e3, false);
    // disable effect
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e4, EVENT_CHAIN_SOLVING);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_operation(d, e4, Some(operation(disop)));
    card::register_effect(d, c, e4, false);
    // disable trap monster
    let e5 = effect::create_effect(d, c);
    effect::set_type(d, e5, EFFECT_TYPE_FIELD);
    effect::set_code(d, e5, EFFECT_DISABLE_TRAPMONSTER);
    effect::set_range(d, e5, LOCATION_MZONE);
    effect::set_target_range(d, e5, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_target(d, e5, Some(effect_card_fn(|d, _e, c| card::is_trap(d, c))));
    card::register_effect(d, c, e5, false);
    aux::double_snare_validity(d, Some(c), LOCATION_MZONE, None);
}
fn disop(d: &mut Duel, a: &EffArgs) {
    let tl = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION])[0].int();
    if tl == LOCATION_SZONE as i64 && effect::is_trap_effect(d, a.re.nil()) {
        duel::negate_effect(d, a.ev as u8);
    }
}
/// `s.aclimit(e,re,tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    card::is_trap(d, effect::get_handler(d, re).nil())
}
