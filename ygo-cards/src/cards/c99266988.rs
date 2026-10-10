//! Chaos Space
use super::prelude::*;

pub const ID: u32 = 99266988;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 1 level 4 to 8 LIGHT/DARK monster, that cannot be normal summoned/set
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SEARCH + CATEGORY_TOHAND);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(thcost)));
    effect::set_target(d, e1, Some(cost_target(thtg)));
    effect::set_operation(d, e1, Some(operation(thop)));
    card::register_effect(d, c, e1, false);
    // Return 1 banished LIGHT/DARK monster to deck, draw 1
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TODECK + CATEGORY_DRAW);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    effect::set_operation(d, e2, Some(operation(tdop)));
    card::register_effect(d, c, e2, false);
}
/// `s.tgfilter(c,tp)`
fn tgfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_monster(d, c)
        && card::is_attribute(d, c, ATTRIBUTE_DARK + ATTRIBUTE_LIGHT)
        && card::is_able_to_grave_as_cost(d, c)
        && {
            let att = card::get_attribute(d, c);
            duel::is_existing_matching_card(d, Some(&|d, x| thfilter1(d, x, att)), tp, LOCATION_DECK, 0, 1, NO_EX)
        }
}
/// `s.thfilter1(c,att)`
fn thfilter1(d: &mut Duel, c: CardId, att: u32) -> bool {
    card::is_able_to_hand(d, c, None)
        && card::is_monster(d, c)
        && card::is_attribute(d, c, ATTRIBUTE_DARK + ATTRIBUTE_LIGHT)
        && card::is_level_above(d, c, 4)
        && card::is_level_below(d, c, 8)
        && !card::is_summonable_card(d, c)
        && !card::is_attribute(d, c, att)
}
/// `s.thcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| tgfilter(d, c, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::discard_hand(d, tp, Some(&|d, c| tgfilter(d, c, tp)), 1, 1, REASON_COST, NO_EX);
    // e:SetLabel(Duel.GetOperatedGroup():GetFirst():GetAttribute())
    let og = duel::get_operated_group(d);
    let tc = group::get_first(d, &og).nil();
    let att = card::get_attribute(d, tc);
    effect::set_label(d, a.e, &[att as i64]);
    true
}
/// `s.thfilter2(c)`
fn thfilter2(d: &mut Duel, c: CardId) -> bool {
    card::is_attribute(d, c, ATTRIBUTE_DARK + ATTRIBUTE_LIGHT)
        && card::is_level_above(d, c, 4)
        && card::is_level_below(d, c, 8)
        && !card::is_summonable_card(d, c)
        && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter2), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let att = effect::get_label(d, a.e) as u32;
    let g1 = duel::select_matching_card(d, tp, Some(&|d, c| thfilter1(d, c, att)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g1.is_empty() {
        duel::send_to_hand(d, g1.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, g1);
    }
}
/// `s.tdfilter(c)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c)
        && card::is_attribute(d, c, ATTRIBUTE_DARK + ATTRIBUTE_LIGHT)
        && card::is_able_to_deck(d, c)
        && !card::is_summonable_card(d, c)
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        // chkc:IsControler(tp) and chkc:IsLocation(LOCATION_REMOVED) and s.filter(chkc)
        // (s.filter is not defined in this script: calling it is a Lua error)
        return card::is_controler(d, chkc, tp)
            && card::is_location(d, chkc, LOCATION_REMOVED)
            && crate::lua_error("attempt to call a nil value (field 'filter')");
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tdfilter), tp, LOCATION_REMOVED, 0, 1, NO_EX) && duel::is_player_can_draw(d, tp, 1);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    let g = duel::select_target(d, tp, Some(&tdfilter), tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX);
    duel::set_target_player(d, tp);
    duel::set_target_param(d, 1);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 3);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, _a: &EffArgs) {
    let tc = duel::first_target(d);
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let (p, n) = (info[0].int() as u8, info[1].int() as u32);
    if let Some(tc) = tc {
        if duel::send_to_deck(d, CardSet::from_card(tc), None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None) == 1 {
            duel::break_effect(d);
            duel::draw(d, p, n, REASON_EFFECT);
        }
    }
}
