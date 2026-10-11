//! Ichiroku's Ledger Book
use super::prelude::*;

pub const ID: u32 = 4825390;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Banish up to 2 of opponent's monsters
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_RECOVER);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn remfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_able_to_remove(d, c, None, None, None)
}
/// Activation legality
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && card::is_able_to_remove(d, chkc, None, None, None);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&remfilter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    let g = duel::select_target(d, a.tp, Some(&remfilter), a.tp, 0, LOCATION_MZONE, 1, 2, false, NO_EX).nil();
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), n, 0, 0);
    true
}
/// Banish up to 2 of opponent's monsters until End Phase
fn activate(d: &mut Duel, a: &EffArgs) {
    let tg = duel::get_target_cards(d, a.e).nil();
    if duel::remove(d, tg.clone(), 0, REASON_EFFECT | REASON_TEMPORARY, None, None) > 0 {
        let c = effect::get_handler(d, a.e).nil();
        for rc in group::iter(d, &tg) {
            let e1 = effect::create_effect(d, c);
            effect::set_description(d, e1, aux::stringid(ID, 1));
            effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
            effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
            effect::set_label_object(d, e1, Value::Card(rc));
            effect::set_count_limit(d, e1, 1, 0, 0, 0);
            effect::set_operation(d, e1, Some(operation(retop)));
            duel::register_effect(d, e1, a.tp);
        }
    }
    // Opponent gains 1000 LP per affected monster
    let og = duel::get_operated_group(d);
    if og.len() > 0 {
        duel::break_effect(d);
        duel::recover(d, 1 - a.tp, og.len() as i64 * 1000, REASON_EFFECT, false, None);
    }
}
fn retop(d: &mut Duel, a: &EffArgs) {
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    duel::return_to_field(d, tc, None, None);
}
