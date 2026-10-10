//! Mind Control
use super::prelude::*;

pub const ID: u32 = 37520316;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_CONTROL);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn controllable(d: &mut Duel, c: CardId) -> bool {
    card::is_controler_can_be_changed(d, c, false, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::get_controler(d, chkc) != a.tp && controllable(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&controllable), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONTROL as u64);
    let g = duel::select_target(d, a.tp, Some(&controllable), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, g, 1, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) && duel::get_control(d, tc, a.tp, PHASE_END as u16, 1, None, None) != 0 {
        let c = effect::get_handler(d, a.e).nil();
        let reset = RESET_EVENT | (RESETS_STANDARD_PHASE_END & !RESET_TURN_SET);
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UNRELEASABLE_SUM);
        effect::set_reset(d, e1, reset, 1);
        effect::set_value(d, e1, 1);
        card::register_effect(d, tc, e1, false);

        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e2, EFFECT_UNRELEASABLE_NONSUM);
        effect::set_reset(d, e2, reset, 1);
        effect::set_value(d, e2, 1);
        card::register_effect(d, tc, e2, false);

        let e3 = effect::create_effect(d, c);
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e3, EFFECT_CANNOT_ATTACK);
        effect::set_reset(d, e3, reset, 1);
        card::register_effect(d, tc, e3, false);
    }
}
