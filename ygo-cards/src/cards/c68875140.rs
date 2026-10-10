//! Hard-sellin' Goblin
use super::prelude::*;

pub const ID: u32 = 68875140;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // todeck
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_TOHAND);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_code(d, e2, EVENT_BATTLE_DAMAGE);
    effect::set_condition(d, e2, Some(condition(cond)));
    effect::set_target(d, e2, Some(cost_target(target)));
    effect::set_operation(d, e2, Some(operation(op)));
    card::register_effect(d, c, e2, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    if a.ep == a.tp {
        return false;
    }
    let eg = a.eg.clone().nil();
    let first = group::get_first(d, &eg).nil();
    card::is_controler(d, first, a.tp)
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::get_sequence(d, c) < 5 && card::is_able_to_hand(d, c, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_SZONE) && card::is_controler(d, chkc, 1 - a.tp) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return true;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, 0, LOCATION_SZONE, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), 1, 0, 0);
    }
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    let tc = duel::first_target(d);
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
        }
    }
}
