//! Des Koala
use super::prelude::*;

pub const ID: u32 = 69579761;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // flip
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE | EFFECT_TYPE_FLIP);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_target_player(d, 1 - a.tp);
    let dam = duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND) * 400;
    duel::set_target_param(d, dam as u32);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, dam);
    true
}
fn op(d: &mut Duel, _a: &EffArgs) {
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    let dam = duel::get_field_group_count(d, p, LOCATION_HAND, 0) as i64 * 400;
    duel::damage(d, p, dam, REASON_EFFECT, false, None);
}
