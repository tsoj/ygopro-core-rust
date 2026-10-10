//! Curse of Fiend
use super::prelude::*;

pub const ID: u32 = 12470447;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Change the battle position of all the monsters on field
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_POSITION);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, TIMING_STANDBY_PHASE, None);
    effect::set_condition(d, e1, Some(condition(|d, _a| duel::is_phase(d, PHASE_STANDBY))));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Can only be activated as Spell Speed 1 at the start of a chain during your Standby Phase while set
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE | EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e2, EFFECT_BECOME_QUICK);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_condition(d, e2, Some(lua_fn(condition2)));
    card::register_effect(d, c, e2, false);
}
/// `function(e)`
fn condition2(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let p = effect::get_handler_player(d, e);
    let c = effect::get_handler(d, e).nil();
    duel::is_turn_player(d, p) && card::is_location(d, c, LOCATION_SZONE) && duel::is_phase(d, PHASE_STANDBY) && duel::get_current_chain(d, false) == 0
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_can_change_position(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_can_change_position(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(g.clone()), g.len() as u8, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_can_change_position(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    if !g.is_empty() {
        duel::change_position(
            d,
            g,
            POS_FACEUP_DEFENSE as u8,
            Some(POS_FACEDOWN_DEFENSE as u8),
            Some(POS_FACEUP_ATTACK as u8),
            Some(POS_FACEUP_ATTACK as u8),
            false,
        );
    }
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_CHANGE_POSITION);
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, tp);
}
