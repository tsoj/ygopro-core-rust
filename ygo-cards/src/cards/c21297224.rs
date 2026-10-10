//! Hysteric Fairy
use super::prelude::*;

pub const ID: u32 = 21297224;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // flip
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_RECOVER);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, None, 2, 2, false, None, NO_EX);
    }
    let g = duel::select_release_group_cost(d, a.tp, None, 2, 2, false, None, NO_EX);
    duel::release(d, g, REASON_COST, None);
    true
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_target_player(d, a.tp);
    duel::set_target_param(d, 1000);
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, 1000);
    true
}
fn op(d: &mut Duel, _a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::recover(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
