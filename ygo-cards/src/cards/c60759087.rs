//! Evil★Twin Present
use super::prelude::*;

pub const ID: u32 = 60759087;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_KI_SIKIL, SET_LIL_LA],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Switch control of 1 "Ki-sikil" monster or 1 "Lil-la" monster you control and 1 face-up monster your opponent controls
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_CONTROL);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMING_END_PHASE));
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    effect::set_target(d, e1, Some(cost_target(conttg)));
    effect::set_operation(d, e1, Some(operation(contop)));
    card::register_effect(d, c, e1, false);
    // Shuffle 1 Set Spell/Trap into the Deck
    let e2 = effect::clone(d, e1);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TODECK);
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    effect::set_operation(d, e2, Some(operation(tdop)));
    card::register_effect(d, c, e2, false);
}
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let mzone = duel::get_field_group(d, a.tp, LOCATION_MZONE, 0);
    let g = group::filter(d, &mzone, &|d, c| card::is_faceup(d, c), NO_EX);
    group::is_exists(d, &g, &|d, c| card::is_set_card(d, c, &[SET_KI_SIKIL as u16]), 1, NO_EX)
        && group::is_exists(d, &g, &|d, c| card::is_set_card(d, c, &[SET_LIL_LA as u16]), 1, NO_EX)
}
fn filter1(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c)
        && (card::is_set_card(d, c, &[SET_KI_SIKIL as u16]) || card::is_set_card(d, c, &[SET_LIL_LA as u16]))
        && card::is_able_to_change_controler(d, c)
}
fn filter2(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_able_to_change_controler(d, c)
}
fn conttg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chkc().is_some() {
        return false;
    }
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter2), tp, 0, LOCATION_MZONE, 1, NO_EX)
            && duel::is_existing_target(d, Some(&filter1), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - tp, desc);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONTROL as u64);
    let mut g1 = duel::select_target(d, tp, Some(&filter1), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONTROL as u64);
    let g2 = duel::select_target(d, tp, Some(&filter2), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    group::add_card(&mut g1, &g2);
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(g1), 2, 0, 0);
    true
}
fn contop(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_target_cards(d, a.e).nil();
    if g.len() != 2 {
        return;
    }
    let cards = group::iter(d, &g);
    let (a_card, b_card) = (cards[0], cards[1]);
    if card::is_relate_to_effect(d, a_card, a.e) && card::is_relate_to_effect(d, b_card, a.e) {
        duel::swap_control(d, a_card, b_card, 0, 0);
    }
}
fn filter3(d: &mut Duel, c: CardId) -> bool {
    card::is_facedown(d, c) && card::is_spell_trap(d, c) && card::is_able_to_deck(d, c)
}
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_ONFIELD) && filter3(d, chkc) && card::is_controler(d, chkc, 1 - tp);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter3), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - tp, desc);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    let g = duel::select_target(d, tp, Some(&filter3), tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), 1, 0, 0);
    true
}
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
