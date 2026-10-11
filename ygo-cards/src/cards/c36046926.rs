//! Sylvan Marshalleaf
use super::prelude::*;

pub const ID: u32 = 36046926;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Excavate up to 2 cards from your Deck
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DECKDES);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    card::register_effect(d, c, e1, false);
    // Destroy 1 monster on the field
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 2));
    effect::set_category(d, e2, CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_condition(d, e2, Some(condition(descon)));
    effect::set_target(d, e2, Some(cost_target(destg)));
    effect::set_operation(d, e2, Some(operation(desop)));
    card::register_effect(d, c, e2, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck(d, a.tp, 1);
    }
    true
}
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if !duel::is_player_can_discard_deck(d, tp, 1) {
        return;
    }
    let ct = duel::get_field_group_count(d, tp, LOCATION_DECK, 0);
    if ct == 0 {
        return;
    }
    let mut ac: i64 = 1;
    if ct > 1 {
        duel::hint(d, HINT_NUMBER as u8, tp, HINTMSG_NUMBER as u64);
        ac = duel::announce_number(d, tp, &[1, 2]).nil().0 as i64;
    }
    duel::confirm_decktop(d, tp, ac as u32);
    let g = duel::get_decktop_group(d, tp, ac as u32);
    let sg = group::filter(d, &g, &|d, c| card::is_race(d, c, RACE_PLANT as u64), NO_EX);
    if !sg.is_empty() {
        duel::disable_shuffle_check(d, true);
        duel::send_to_grave(d, sg.clone(), REASON_EFFECT | REASON_EXCAVATE, None, None);
    }
    ac -= sg.len() as i64;
    if ac > 0 {
        duel::move_decktop_to_bottom(d, ac as u32, tp);
        duel::sort_deckbottom(d, tp, tp, ac as u16);
    }
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, c, LOCATION_DECK) && card::is_reason(d, c, REASON_EXCAVATE)
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE);
    }
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    let g = duel::select_target(d, tp, None, tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
