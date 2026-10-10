//! Ameba
use super::prelude::*;

pub const ID: u32 = 95174353;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // damage
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_CONTROL_CHANGED);
    effect::set_property(d, e1, EFFECT_FLAG_NO_TURN_RESET, 0);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return !card::is_status(d, c, STATUS_CHAINING);
    }
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, a.tp, 2000);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    duel::damage(d, a.tp, 2000, REASON_EFFECT, false, None);
}
