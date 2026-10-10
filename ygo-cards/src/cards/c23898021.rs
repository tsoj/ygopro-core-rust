//! Lilith, Lady of Lament
use super::prelude::*;

pub const ID: u32 = 23898021;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atk
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_operation(d, e1, Some(operation(regop)));
    card::register_effect(d, c, e1, false);
    // search
    let e2 = effect::create_effect(d, c);
    effect::set_category(d, e2, CATEGORY_SET);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_cost(d, e2, Some(cost_target(thcost)));
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
}
fn regop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_SET_BASE_ATTACK);
    effect::set_value(d, e1, 1000);
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
    card::register_effect(d, c, e1, false);
}
fn dark_filter(d: &mut Duel, c: CardId) -> bool {
    card::is_attribute(d, c, ATTRIBUTE_DARK)
}
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&dark_filter), 1, 1, false, None, NO_EX);
    }
    let g = duel::select_release_group_cost(d, a.tp, Some(&dark_filter), 1, 1, false, None, NO_EX);
    duel::release(d, g, REASON_COST, None);
    true
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_normal_trap(d, c) && card::is_s_setable(d, c, false, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 3, NO_EX);
    }
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_matching_group(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, NO_EX);
    if g.len() >= 3 {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
        let sg = group::select(d, &g, a.tp, 3, 3, false, NO_EX).nil();
        duel::confirm_cards(d, 1 - a.tp, sg.clone());
        duel::shuffle_deck(d, a.tp);
        duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_SET as u64);
        let tg = group::select(d, &sg, 1 - a.tp, 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &tg).nil();
        duel::s_set(d, a.tp, tc, Some(a.tp), false);
    }
}
