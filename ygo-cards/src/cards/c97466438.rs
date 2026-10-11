//! Zany Zebra
use super::prelude::*;

pub const ID: u32 = 97466438;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // pendulum summon
    pendulum::add_procedure(d, c, Some(false), None);
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, 1160);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_cost(d, e1, Some(cost_target(reg)));
    card::register_effect(d, c, e1, false);
    // disable zone
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    effect::set_target(d, e2, Some(cost_target(ztg)));
    effect::set_operation(d, e2, Some(operation(zop2)));
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e3, false);
    // disable zone
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 1));
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e4, LOCATION_PZONE);
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    effect::set_condition(d, e4, Some(condition(zcon)));
    effect::set_target(d, e4, Some(cost_target(ztg)));
    effect::set_operation(d, e4, Some(operation(zop)));
    card::register_effect(d, c, e4, false);
}
fn reg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let c = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, c, ID, RESET_PHASE | PHASE_END, EFFECT_FLAG_OATH, 1, 0, 0);
    true
}
fn zcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, c, ID) != 0
}
fn ztg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let sum = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, Some(PLAYER_NONE as u8), Some(0), None).0
            + duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, Some(PLAYER_NONE as u8), Some(0), None).0
            + duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, Some(PLAYER_NONE as u8), Some(0), None).0
            + duel::get_location_count(d, 1 - a.tp, LOCATION_SZONE as u8, Some(PLAYER_NONE as u8), Some(0), None).0;
        return sum > 0;
    }
    let dis = duel::select_disable_field(d, a.tp, 1, LOCATION_MZONE | LOCATION_SZONE, LOCATION_MZONE | LOCATION_SZONE, Some(0), false);
    duel::hint(d, HINT_ZONE as u8, a.tp, dis as u64);
    effect::set_label(d, a.e, &[dis as i64]);
    true
}
fn zop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let label = effect::get_label(d, a.e);
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_range(d, e1, LOCATION_PZONE);
    effect::set_code(d, e1, EFFECT_DISABLE_FIELD);
    effect::set_operation(d, e1, Some(lua_fn(disop)));
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    effect::set_label(d, e1, &[label]);
    card::register_effect(d, c, e1, false);
}
fn zop2(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let label = effect::get_label(d, a.e);
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EFFECT_DISABLE_FIELD);
    effect::set_operation(d, e1, Some(lua_fn(disop)));
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    effect::set_label(d, e1, &[label]);
    card::register_effect(d, c, e1, false);
}
/// `s.disop(e,tp)`: returns the label (the zones)
fn disop(d: &mut Duel, a: Args) -> i64 {
    effect::get_label(d, a.e(0))
}
