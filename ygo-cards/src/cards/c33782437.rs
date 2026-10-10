//! One Day of Peace
use super::prelude::*;

pub const ID: u32 = 33782437;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_category(d, e1, CATEGORY_DRAW);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1) && duel::is_player_can_draw(d, 1 - a.tp, 1);
    }
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, PLAYER_ALL as u8, 1);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let d1 = duel::draw(d, a.tp, 1, REASON_EFFECT);
    let d2 = duel::draw(d, 1 - a.tp, 1, REASON_EFFECT);
    if d1 == 0 || d2 == 0 {
        return;
    }
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CHANGE_DAMAGE);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e1, 1, 1);
    effect::set_value(d, e1, 0);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 2);
    duel::register_effect(d, e1, a.tp);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_NO_EFFECT_DAMAGE);
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 2);
    duel::register_effect(d, e2, a.tp);
    let h = effect::get_handler(d, a.e).nil();
    let e3 = effect::create_effect(d, h);
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET | EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 2);
    effect::set_target_range(d, e3, 1, 1);
    duel::register_effect(d, e3, a.tp);
}
