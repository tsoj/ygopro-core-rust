//! Sanguine Swamp
use super::prelude::*;

pub const ID: u32 = 60627999;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_POSITION);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    card::register_effect(d, c, e1, false);
    // destroy
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_code(d, e2, EFFECT_SELF_DESTROY);
    effect::set_condition(d, e2, Some(condition(sdescon)));
    card::register_effect(d, c, e2, false);
    // disable spsummon
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_property(d, e3, EFFECT_FLAG_SET_AVAILABLE, 0);
    effect::set_code(d, e3, EFFECT_CANNOT_TRIGGER);
    effect::set_target_range(d, e3, LOCATION_SZONE, LOCATION_SZONE);
    effect::set_target(d, e3, Some(effect_card_fn(|d, _e, c| acttg(d, c))));
    card::register_effect(d, c, e3, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let c = effect::get_handler(d, a.e).nil();
    card::set_turn_counter(d, c, 0);
    // destroy
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_range(d, e1, LOCATION_SZONE);
    effect::set_condition(d, e1, Some(condition(descon)));
    effect::set_operation(d, e1, Some(operation(desop)));
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_STANDBY | RESET_SELF_TURN, 2);
    card::register_effect(d, c, e1, false);
    true
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    a.tp == duel::get_turn_player(d)
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let ct = card::get_turn_counter(d, c) + 1;
    card::set_turn_counter(d, c, ct as u16);
    if ct == 2 {
        duel::destroy(d, c, REASON_EFFECT, None, None);
    }
}
fn sdescon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&card::is_spell_trap), p, LOCATION_ONFIELD, 0, 1, c.into())
}
fn acttg(d: &mut Duel, c: CardId) -> bool {
    card::is_facedown(d, c) && card::get_sequence(d, c) != 5
}
