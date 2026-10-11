//! Maliss in the Mirror
use super::prelude::*;

pub const ID: u32 = 93453053;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MALISS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Banish 1 "Maliss" monster from your hand or field and negate 1 face-up opponent's monster's effects
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_DISABLE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(rmdistg)));
    effect::set_operation(d, e1, Some(operation(rmdisop)));
    card::register_effect(d, c, e1, false);
    // Banish 1 "Maliss" card from your GY and add 1 "Maliss" card with the same type (Monster, Spell, or Trap) from your Deck to your hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_REMOVE + CATEGORY_SEARCH + CATEGORY_TOHAND);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_REMOVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_target(d, e2, Some(cost_target(rmthtg)));
    effect::set_operation(d, e2, Some(operation(rmthop)));
    card::register_effect(d, c, e2, false);
}
fn rmdisfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_MALISS as u16])
        && card::is_monster(d, c)
        && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_HAND))
        && card::is_able_to_remove(d, c, None, None, None)
}
fn rmdistg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_controler(d, chkc, 1 - tp)
            && card::is_negatable_monster(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_negatable_monster(d, c)), tp, 0, LOCATION_MZONE, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&rmdisfilter), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_NEGATE as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_negatable_monster(d, c)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, tp, (LOCATION_HAND | LOCATION_MZONE) as i32);
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, g, 1, tp, 0);
    true
}
fn rmdisop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let tc = duel::first_target(d);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&rmdisfilter), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty()
        && duel::remove(d, &g, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0
        && card::is_location(d, group::get_first(d, &g).nil(), LOCATION_REMOVED)
        && card::is_relate_to_effect(d, tc.nil(), e)
        && card::is_faceup(d, tc.nil())
    {
        let h = effect::get_handler(d, e).nil();
        card::negate_effects(d, tc.nil(), h, Some(RESET_PHASE | PHASE_END), false, None);
    }
}
fn rmthfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_MALISS as u16])
        && card::is_able_to_remove(d, c, None, None, None)
        && {
            let typ = card::get_main_card_type(d, c);
            duel::is_existing_matching_card(d, Some(&|d, cc| thfilter(d, cc, typ)), tp, LOCATION_DECK, 0, 1, NO_EX)
        }
}
fn thfilter(d: &mut Duel, c: CardId, typ: u32) -> bool {
    card::is_set_card(d, c, &[SET_MALISS as u16]) && card::is_type(d, c, typ) && card::is_able_to_hand(d, c, None)
}
fn rmthtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && rmthfilter(d, chkc, tp);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| rmthfilter(d, c, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| rmthfilter(d, c, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, tp, 0);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
fn rmthop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let tc = duel::first_target(d).nil();
    if !card::is_relate_to_effect(d, tc, e)
        || duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None) == 0
        || !card::is_location(d, tc, LOCATION_REMOVED)
    {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let typ = card::get_main_card_type(d, tc);
    let g = duel::select_matching_card(d, tp, Some(&|d, cc| thfilter(d, cc, typ)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, &g);
    }
}
