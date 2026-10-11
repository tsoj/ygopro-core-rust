//! Gunkan Suship Catch-of-the-Day
use super::prelude::*;

pub const ID: u32 = 83008724;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GUNKAN], listed_names: &[CARD_SUSHIP_SHARI], ..ScriptInfo::DEFAULT };

const SUSHIP_NAMES: &[u32] = &[61027400, 42377643, 78362751];

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_counter_permit(d, c, 0x20d, None, None);
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Effects
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_TODECK);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_cost(d, e2, Some(cost_target(cost_fn)));
    effect::set_target(d, e2, Some(cost_target(target)));
    effect::set_operation(d, e2, Some(operation(op)));
    card::register_effect(d, c, e2, false);
    // Register a flag before it is destroyed
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e3, EVENT_LEAVE_FIELD_P);
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_operation(d, e3, Some(operation(|d, a: &EffArgs| {
        let c = effect::get_handler(d, a.e).nil();
        let n = card::get_counter(d, c, 0x20d);
        effect::set_label(d, a.e, &[n as i64]);
    })));
    card::register_effect(d, c, e3, false);
    // Make the opponent pay LP
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 1));
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e4, EVENT_DESTROYED);
    effect::set_condition(d, e4, Some(condition(lpcond)));
    effect::set_operation(d, e4, Some(operation(lpop)));
    effect::set_label_object(d, e4, Value::Effect(e3));
    card::register_effect(d, c, e4, false);
}
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, &[CARD_SUSHIP_SHARI]) && !card::is_public(d, c)
}
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&costfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONFIRM as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&costfilter), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    duel::confirm_cards(d, 1 - a.tp, &g);
    duel::shuffle_hand(d, a.tp);
    true
}
fn shipfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_GUNKAN as u16]) && card::is_type(d, c, TYPE_XYZ) && !card::is_public(d, c)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&shipfilter), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, Some(c.into()), 1, a.tp, 0x20d);
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_TODECK, Some(c.into()), 1, 0, 0);
    true
}
fn thfilter_code(d: &mut Duel, c: CardId, code: u32) -> bool {
    card::is_code(d, c, &[code]) && card::is_able_to_hand(d, c, None)
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if !card::add_counter(d, c, 0x20d, 1, false) {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    let g = duel::select_matching_card(d, tp, Some(&shipfilter), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    if g.is_empty() {
        return;
    }
    duel::confirm_cards(d, 1 - tp, &g);
    duel::shuffle_extra(d, tp);
    let res = duel::select_cards_from_codes(d, 1 - tp, 1, 1, false, SUSHIP_NAMES).nil();
    let code = res[0].0;
    duel::hint(d, HINT_CARD as u8, 1 - tp, code as u64);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let sc = duel::select_matching_card(
        d,
        tp,
        Some(&|d, x| thfilter_code(d, x, code)),
        tp,
        LOCATION_DECK,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    if !sc.is_empty() {
        duel::send_to_hand(d, &sc, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, &sc);
    } else if card::is_relate_to_effect(d, c, a.e) {
        duel::send_to_deck(d, c, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
fn lpcond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_previous_controler(d, c, a.tp) && card::is_reason(d, c, REASON_DESTROY) && card::get_reason_player(d, c) == 1 - a.tp
}
fn lpop(d: &mut Duel, a: &EffArgs) {
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    let value = effect::get_label(d, lo) * 500;
    if value > 0 && duel::get_lp(d, 1 - a.tp) as i64 >= value {
        duel::pay_lp_cost(d, 1 - a.tp, value as u32);
    }
}
