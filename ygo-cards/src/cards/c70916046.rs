//! Linear Equation Cannon
use super::prelude::*;

pub const ID: u32 = 70916046;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DECKDES + CATEGORY_TODECK + CATEGORY_TOGRAVE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_BATTLE_START | TIMING_BATTLE_END));
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_condition(d, e1, Some(condition(|d, _a: &EffArgs| duel::is_battle_phase(d, None))));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_EFFECT) && card::has_level(d, c)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), tp, 0, LOCATION_MZONE, 1, NO_EX)
            && duel::get_field_group_count(d, tp, LOCATION_DECK, 0) > 0;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_NUMBER as u64);
    let ac = duel::announce_number(d, tp, &[1, 2, 3, 4, 5, 6]).nil().0 as i64;
    effect::set_label(d, a.e, &[ac]);
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_TODECK, None, 1, 1 - tp, LOCATION_ONFIELD as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    let g = duel::select_matching_card(d, tp, Some(&filter), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    let Some(tc) = group::get_first(d, &g) else {
        return;
    };
    duel::hint_selection(d, tc, true);
    let dc = effect::get_label(d, a.e);
    let ct = card::get_level(d, tc) as i64 * dc + duel::get_field_group_count(d, tp, 0, LOCATION_ONFIELD) as i64;
    if ct == duel::get_field_group_count(d, tp, LOCATION_GRAVE, 0) as i64 {
        let mut ac = dc;
        if ac > 1 {
            let mut tbl: Vec<u64> = Vec::new();
            for i in (1..=dc).rev() {
                if duel::is_player_can_discard_deck(d, tp, i as u32) {
                    tbl.push(i as u64);
                }
            }
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_NUMBER as u64);
            ac = duel::announce_number(d, tp, &tbl).nil().0 as i64;
        }
        if duel::discard_deck(d, tp, ac as u16, REASON_EFFECT) == 0 {
            return;
        }
        let og = duel::get_operated_group(d);
        let ct2 = group::filter_count(d, &og, &|d, c| card::is_location(d, c, LOCATION_GRAVE), NO_EX);
        if ct2 == 0 {
            return;
        }
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
        let g2 = duel::select_matching_card(d, tp, Some(&card::is_able_to_deck), tp, 0, LOCATION_ONFIELD, 1, ct2 as u16, false, NO_EX).nil();
        if !g2.is_empty() {
            duel::hint_selection(d, g2.clone(), true);
            duel::send_to_deck(d, &g2, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        }
    } else {
        let lp = duel::get_lp(d, tp) as i64 - dc * 500;
        duel::set_lp(d, tp, lp.max(0) as i32);
    }
}
