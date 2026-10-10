//! Ookazi
use super::prelude::*;

pub const ID: u32 = 19523799;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // damage
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(damtg)));
    effect::set_operation(d, e1, Some(operation(damop)));
    card::register_effect(d, c, e1, false);
}
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_target_player(d, 1 - a.tp);
    duel::set_target_param(d, 800);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 800);
    true
}
fn damop(d: &mut Duel, _a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
