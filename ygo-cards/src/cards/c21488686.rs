//! Psychic Rejuvenation
use super::prelude::*;

pub const ID: u32 = 21488686;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // recover
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_category(d, e1, CATEGORY_RECOVER);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn psychic_faceup(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_race(d, c, RACE_PSYCHIC)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&psychic_faceup), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::set_target_player(d, a.tp);
    let rec = duel::get_matching_group_count(d, Some(&psychic_faceup), a.tp, LOCATION_MZONE, 0, NO_EX) as i64 * 1000;
    duel::set_target_param(d, rec as u32);
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, rec as i32);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let rec = duel::get_matching_group_count(d, Some(&psychic_faceup), a.tp, LOCATION_MZONE, 0, NO_EX) as i64 * 1000;
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER]);
    let p = info[0].int() as u8;
    duel::recover(d, p, rec, REASON_EFFECT, false, None);
}
