//! Burning Land
use super::prelude::*;

pub const ID: u32 = 24294108;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // damage
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_DAMAGE);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD | EFFECT_TYPE_TRIGGER_F);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_code(d, e2, EVENT_PHASE | PHASE_STANDBY);
    effect::set_target(d, e2, Some(cost_target(damtg)));
    effect::set_operation(d, e2, Some(operation(damop)));
    card::register_effect(d, c, e2, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let g = duel::get_matching_group(d, None, 0, LOCATION_FZONE, LOCATION_FZONE, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), g.len() as u8, 0, 0);
    true
}
fn activate(d: &mut Duel, _a: &EffArgs) {
    let g = duel::get_matching_group(d, None, 0, LOCATION_FZONE, LOCATION_FZONE, NO_EX);
    if !g.is_empty() {
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let cp = duel::get_turn_player(d);
    duel::set_target_player(d, cp);
    duel::set_target_param(d, 500);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, cp, 500);
    true
}
fn damop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
