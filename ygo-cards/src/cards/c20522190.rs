//! Dark Mirror Force
use super::prelude::*;

pub const ID: u32 = 20522190;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_REMOVE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp)
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_defense_pos(d, c) && card::is_able_to_remove(d, c, None, None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    let g = duel::get_matching_group(d, Some(&filter), a.tp, 0, LOCATION_MZONE, NO_EX);
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), n, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_matching_group(d, Some(&filter), a.tp, 0, LOCATION_MZONE, NO_EX);
    if !g.is_empty() {
        duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
