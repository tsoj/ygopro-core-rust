//! Drop Off
use super::prelude::*;

pub const ID: u32 = 55773067;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_HANDES);
    effect::set_code(d, e1, EVENT_DRAW);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cond(_d: &mut Duel, a: &EffArgs) -> bool {
    a.ep != a.tp && (a.r & REASON_RULE) != 0
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_target_card(d, a.eg());
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, 1 - a.tp, 1);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let sg = group::filter(d, &a.eg.clone().nil(), &|d, c| card::is_relate_to_effect(d, c, e), NO_EX);
    if sg.is_empty() {
    } else if sg.len() == 1 {
        duel::send_to_grave(d, sg, REASON_EFFECT | REASON_DISCARD, None, None);
    } else {
        duel::hint(d, HINT_SELECTMSG as u8, a.ep, HINTMSG_TOGRAVE as u64);
        let dg = group::select(d, &sg, a.ep, 1, 1, false, NO_EX).nil();
        duel::send_to_grave(d, dg, REASON_EFFECT | REASON_DISCARD, None, None);
    }
}
