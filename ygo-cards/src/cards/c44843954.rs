//! Sweet Dreams, Nemleria
use super::prelude::*;

pub const ID: u32 = 44843954;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_DREAMING_NEMLERIA], listed_series: &[SET_NEMLERIA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Search 1 "Nemleria" monster and prevent the opponent's effects when a "Nemleria" is Summoned
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(thtg)));
    effect::set_operation(d, e1, Some(operation(thop)));
    card::register_effect(d, c, e1, false);
    // Place 1 "Dreaming Nemleria" in the Extra Deck
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOEXTRA);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_cost(d, e2, Some(cost_target(tedcost)));
    effect::set_target(d, e2, Some(cost_target(tedtg)));
    effect::set_operation(d, e2, Some(operation(tedop)));
    card::register_effect(d, c, e2, false);
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_NEMLERIA as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn is_dreaming_faceup(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_code(d, c, &[CARD_DREAMING_NEMLERIA])
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if g.is_empty() || duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) == 0 {
        return;
    }
    duel::confirm_cards(d, 1 - tp, g);
    if !duel::is_existing_matching_card(d, Some(&is_dreaming_faceup), tp, LOCATION_EXTRA, 0, 1, NO_EX) {
        return;
    }
    duel::break_effect(d);
    let c = effect::get_handler(d, a.e).nil();
    // Prevent activations when a "Nemleria" monster is Normal or Special Summoned
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_operation(d, e1, Some(operation(sumop)));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, tp);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    duel::register_effect(d, e2, tp);
    aux::register_client_hint(d, c, 0, tp, 1, 0, Some(aux::stringid(ID, 2)), 0, None);
}
fn sumop(d: &mut Duel, a: &EffArgs) {
    let eg = a.eg.clone().nil();
    if group::is_exists(d, &eg, &|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_NEMLERIA as u16]), 1, NO_EX) {
        duel::set_chain_limit_till_chain_end(d, lua_fn(|_d, a: Args| a.p(2) == a.p(1)));
    }
}
fn tedcost(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_deck_as_cost(d, c);
    }
    duel::send_to_deck(d, CardSet::from_card(c), None, SEQ_DECKBOTTOM as i32, REASON_COST, None);
    true
}
fn tedfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, &[CARD_DREAMING_NEMLERIA]) && card::is_faceup(d, c) && card::is_able_to_extra(d, c) && !card::is_forbidden(d, c)
}
fn tedtg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, a.tp) && tedfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tedfilter), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    let g = duel::select_target(d, a.tp, Some(&tedfilter), a.tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_TOEXTRA, g, 1, a.tp, 0);
    true
}
fn tedop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::send_to_extra_p(d, CardSet::from_card(tc), Some(a.tp), REASON_EFFECT, None);
    }
}
