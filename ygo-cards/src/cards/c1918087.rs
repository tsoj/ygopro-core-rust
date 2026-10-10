//! Minor Goblin Official
use super::prelude::*;

pub const ID: u32 = 1918087;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_DRAW_PHASE));
    effect::set_condition(d, e1, Some(condition(actcon)));
    card::register_effect(d, c, e1, false);
    // damage
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_DAMAGE);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_code(d, e2, EVENT_PHASE | PHASE_STANDBY);
    effect::set_condition(d, e2, Some(condition(damcon)));
    effect::set_target(d, e2, Some(cost_target(damtg)));
    effect::set_operation(d, e2, Some(operation(damop)));
    card::register_effect(d, c, e2, false);
}
fn actcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_lp(d, 1 - a.tp) <= 3000
}
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp)
}
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_target_player(d, 1 - a.tp);
    duel::set_target_param(d, 500);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 500);
    true
}
fn damop(d: &mut Duel, _a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
