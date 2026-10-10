//! Swords of Concealing Light
use super::prelude::*;

pub const ID: u32 = 12923641;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_POSITION + CATEGORY_SET);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Prevent change in positions
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_code(d, e2, EFFECT_CANNOT_CHANGE_POSITION);
    effect::set_property(d, e2, EFFECT_FLAG_SET_AVAILABLE, 0);
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    card::register_effect(d, c, e2, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let c = effect::get_handler(d, a.e).nil();
    card::set_turn_counter(d, c, 0);
    let g = duel::get_matching_group(d, Some(&card::is_can_turn_set), a.tp, 0, LOCATION_MZONE, NO_EX);
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(g), n, a.tp, POS_FACEDOWN_DEFENSE as i32);
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
fn activate(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let g = duel::get_matching_group(d, Some(&card::is_can_turn_set), a.tp, 0, LOCATION_MZONE, NO_EX);
    if !g.is_empty() {
        duel::change_position(d, g, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
    }
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, a.tp)
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let ct = card::get_turn_counter(d, c) + 1;
    card::set_turn_counter(d, c, ct as u16);
    if ct == 2 {
        duel::destroy(d, c, REASON_RULE, None, None);
    }
}
