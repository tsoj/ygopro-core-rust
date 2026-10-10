//! Nekroz Divinemirror
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 50596425;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NEKROZ], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Ritual Summon 1 "Nekroz" Ritual Monster from your hand or banishment, by Tributing monsters from your hand and/or field, and/or sending "Nekroz" monsters from your Extra Deck to the GY, whose total Levels equal or exceed its Level
    let e1 = ritual::create_proc(
        d,
        c,
        ritual::RitualProc {
            lvtype: RITPROC_GREATER,
            filter: Some(Arc::new(|d: &mut Duel, x: CardId| card::is_set_card(d, x, &[SET_NEKROZ as u16]))),
            extrafil: Some(Arc::new(|d: &mut Duel, a: &EffArgs| Some(duel::get_matching_group(d, Some(&extrafilter), a.tp, LOCATION_EXTRA, 0, NO_EX)))),
            location: Some(LOCATION_HAND | LOCATION_REMOVED),
            ..Default::default()
        },
    );
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_hint_timing(d, e1, 0, Some((TIMING_MAIN_END | TIMINGS_CHECK_MONSTER) as u32));
    effect::set_condition(d, e1, Some(condition(main_phase_cond)));
    card::register_effect(d, c, e1, false);
    // Add 1 "Nekroz" Spell from your Deck to your hand
    let e2 = effect::create_effect(d, c);
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_cost(d, e2, Some(cost_target(thcost)));
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
}
fn main_phase_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_main_phase(d, None)
}
fn extrafilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_NEKROZ as u16]) && card::has_level(d, c) && card::is_able_to_grave(d, c, None)
}
fn thcostfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_NEKROZ as u16]) && card::is_monster(d, c) && card::is_able_to_remove_as_cost(d, c, None)
}
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_remove_as_cost(d, c, None) && duel::is_existing_matching_card(d, Some(&thcostfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    let mut g = duel::select_matching_card(d, tp, Some(&thcostfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    group::add_card(&mut g, &CardSet::from_card(c));
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_NEKROZ as u16]) && card::is_spell(d, c) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, &g);
    }
}
