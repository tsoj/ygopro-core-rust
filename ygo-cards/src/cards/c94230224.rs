//! Needle Ball
use super::prelude::*;

pub const ID: u32 = 94230224;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // flip
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_DELAY, 0);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP + EFFECT_TYPE_TRIGGER_O);
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(2000))));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_target_player(d, 1 - a.tp);
    duel::set_target_param(d, 1000);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 1000);
    true
}
fn op(d: &mut Duel, _a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
