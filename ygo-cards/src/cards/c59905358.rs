//! Dice Try!
use super::prelude::*;

pub const ID: u32 = 59905358;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    if !duel::is_chain_negatable(d, a.ev as u8) {
        return false;
    }
    if !effect::is_monster_effect(d, a.re.nil()) && !effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE) {
        return false;
    }
    duel::get_operation_info(d, a.ev, CATEGORY_DICE).is_some()
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    let re = a.re.nil();
    let h = effect::get_handler(d, re).nil();
    if card::is_destructable(d, h, None) && card::is_relate_to_effect(d, h, re) {
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    if duel::negate_activation(d, a.ev as u8) {
        let re = a.re.nil();
        let h = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, h, re) {
            duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
        }
    }
}
