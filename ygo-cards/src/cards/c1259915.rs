//! Sacred Beasts Thunderclap
use super::prelude::*;

pub const ID: u32 = 1259915;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[1259915, 65861210], listed_series: &[SET_SACRED_BEAST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e0, false);
    // During your Main Phase: You can place 2 "Sacred Beasts Thunderclap" from your hand, Deck, and/or GY face-up on your field, then you can reveal 1 Level 10 "Sacred Beast" monster in your hand, then place 1 "Fallen Paradise of the Sacred Beasts" from your Deck face-up in your Field Zone
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_SZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(pltg)));
    effect::set_operation(d, e1, Some(operation(plop)));
    card::register_effect(d, c, e1, false);
    // During your opponent's End Phase, if this card is in your GY: You can add it to your hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOHAND);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(cond)));
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
}
fn selfplfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_code(d, c, &[ID]) && !card::is_forbidden(d, c) && card::check_unique_on_field(d, c, tp, None, None)
}
fn pltg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let tp = a.tp;
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 >= 2
            && duel::is_existing_matching_card(d, Some(&|d, c| selfplfilter(d, c, tp)), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 2, NO_EX);
    }
    true
}
fn revealfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_level(d, c, &[10]) && card::is_set_card(d, c, &[SET_SACRED_BEAST as u16]) && !card::is_public(d, c)
}
fn fieldplfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_code(d, c, &[65861210]) && !card::is_forbidden(d, c) && card::check_unique_on_field(d, c, tp, None, None)
}
fn plop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 < 2 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&aux::necro_valley_filter(|d, c| selfplfilter(d, c, tp))),
        tp,
        LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE,
        0,
        2,
        2,
        false,
        NO_EX,
    )
    .nil();
    if g.len() == 2 {
        let order = group::iter(d, &g);
        duel::move_to_field(d, order[0], tp, tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None);
        duel::move_to_field(d, order[1], tp, tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None);
        if duel::is_existing_matching_card(d, Some(&revealfilter), tp, LOCATION_HAND, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| fieldplfilter(d, c, tp)), tp, LOCATION_DECK, 0, 1, NO_EX)
            && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
        {
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
            let rg = duel::select_matching_card(d, tp, Some(&revealfilter), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
            if rg.is_empty() {
                return;
            }
            duel::break_effect(d);
            duel::confirm_cards(d, 1 - tp, &rg);
            duel::shuffle_hand(d, tp);
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
            let sc = duel::select_matching_card(d, tp, Some(&|d, c| fieldplfilter(d, c, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
            if let Some(sc) = group::get_first(d, &sc) {
                duel::break_effect(d);
                duel::move_to_field(d, sc, tp, tp, LOCATION_FZONE as u16, POS_FACEUP as u8, true, None);
            }
        }
    }
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::send_to_hand(d, c, None, REASON_EFFECT, None);
    }
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp)
}
