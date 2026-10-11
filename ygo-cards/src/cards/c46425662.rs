//! Elementsaber Nalu
use super::prelude::*;

pub const ID: u32 = 46425662;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ELEMENTSABER, SET_ELEMENTAL_LORD], listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // spsummon
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(thcost)));
    effect::set_target(d, e1, Some(cost_target(thtg)));
    effect::set_operation(d, e1, Some(operation(thop)));
    card::register_effect(d, c, e1, false);
    // att change
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_target(d, e2, Some(cost_target(atttg)));
    effect::set_operation(d, e2, Some(operation(attop)));
    card::register_effect(d, c, e2, false);
}
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_ELEMENTSABER as u16]) && card::is_monster(d, c) && card::is_able_to_grave_as_cost(d, c)
}
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let mut fg = CardSet::new();
    if let Some(pe) = duel::is_player_affected_by_effect(d, tp, 61557074) {
        group::add_card(&mut fg, &CardSet::from_card(effect::get_handler(d, pe).nil()));
    }
    let mut loc = LOCATION_HAND;
    if !fg.is_empty() {
        loc = LOCATION_HAND | LOCATION_DECK;
    }
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&costfilter), tp, loc, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    let sel = duel::select_matching_card(d, tp, Some(&costfilter), tp, loc, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel).nil();
    if card::is_location(d, tc, LOCATION_DECK) {
        let fc = if fg.len() == 1 {
            group::get_first(d, &fg).nil()
        } else {
            let sel_fc = group::select(d, &fg, tp, 1, 1, false, NO_EX).nil();
            group::get_first(d, &sel_fc).nil()
        };
        let fc_code = card::get_code(d, fc)[0];
        duel::hint(d, HINT_CARD as u8, 0, fc_code as u64);
        card::register_flag_effect(d, fc, 61557074, RESETS_STANDARD_PHASE_END, 0, 0, 0, 0);
    }
    duel::send_to_grave(d, tc, REASON_COST, None, None);
    true
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    (card::is_set_card(d, c, &[SET_ELEMENTSABER as u16]) || card::is_set_card(d, c, &[SET_ELEMENTAL_LORD as u16]))
        && card::is_monster(d, c)
        && !card::is_code(d, c, &[ID])
        && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && thfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_target(d, tp, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, 0, 0);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
    }
}
fn atttg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let c = effect::get_handler(d, a.e).nil();
    let att = card::announce_another_attribute(d, c, a.tp);
    effect::set_label(d, a.e, &[att as i64]);
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(c)), 1, a.tp, LOCATION_GRAVE as i32);
    true
}
fn attop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Change Attribute
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_CHANGE_ATTRIBUTE);
    effect::set_value(d, e1, effect::get_label(d, a.e));
    effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
    card::register_effect(d, c, e1, false);
}
