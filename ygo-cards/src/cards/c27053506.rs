//! Secret Barrel
use super::prelude::*;

pub const ID: u32 = 27053506;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, None, a.tp, 0, LOCATION_HAND | LOCATION_ONFIELD, 1, NO_EX);
    }
    duel::set_target_player(d, 1 - a.tp);
    let dam = duel::get_field_group_count(d, 1 - a.tp, LOCATION_HAND | LOCATION_ONFIELD, 0) * 200;
    duel::set_target_param(d, dam as u32);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, dam);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    let dam = duel::get_field_group_count(d, 1 - a.tp, LOCATION_HAND | LOCATION_ONFIELD, 0) * 200;
    duel::damage(d, p, dam as i64, REASON_EFFECT, false, None);
}
