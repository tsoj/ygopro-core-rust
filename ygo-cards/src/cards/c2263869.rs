//! Ultimate Slayer
use super::prelude::*;

pub const ID: u32 = 2263869;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Return to the Deck
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TODECK);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_cost(d, e1, Some(cost_target(tdcost)));
    effect::set_target(d, e1, Some(cost_target(tdtg)));
    effect::set_operation(d, e1, Some(operation(tdop)));
    card::register_effect(d, c, e1, false);
}
fn tdcostfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    if !card::is_able_to_grave_as_cost(d, c) {
        return false;
    }
    let extype = card::get_type(d, c) & (TYPE_EXTRA | TYPE_PENDULUM);
    duel::is_existing_matching_card(d, Some(&|d, cc| tdfilter(d, cc, e, extype)), tp, 0, LOCATION_MZONE, 1, NO_EX)
}
fn tdfilter(d: &mut Duel, c: CardId, e: EffectId, extype: u32) -> bool {
    card::is_faceup(d, c) && card::is_type(d, c, extype) && card::is_able_to_deck(d, c) && card::is_can_be_effect_target(d, c, Some(e))
}
fn tdcost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| tdcostfilter(d, c, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| tdcostfilter(d, c, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    duel::send_to_grave(d, g.clone(), REASON_COST, None, None);
    let first = group::get_first(d, &g).nil();
    let label = card::get_type(d, first) & (TYPE_EXTRA | TYPE_PENDULUM);
    effect::set_label(d, a.e, &[label as i64]);
    true
}
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let extype = effect::get_label(d, a.e) as u32;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && tdfilter(d, chkc, a.e, extype);
    }
    if a.chk == Some(0) {
        return true;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    let g = duel::select_target(d, a.tp, Some(&|d, c| tdfilter(d, c, a.e, extype)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_TODECK, g, 1, 0, 0);
    if effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        duel::set_chain_limit(d, lua_fn(|d, x| x.p(1) == x.p(2) || !effect::is_monster_effect(d, x.e(0))));
    }
    true
}
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
