//! Adamancipator Laputite
use super::prelude::*;

pub const ID: u32 = 46552140;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ADAMANCIPATOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // atk up
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e2, LOCATION_FZONE);
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // aux.TargetBoolFunction(Card.IsRace,RACE_ROCK)
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_race(d, c, RACE_ROCK))));
    effect::set_value(d, e2, 500);
    card::register_effect(d, c, e2, false);
    // def up
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    card::register_effect(d, c, e3, false);
    // hack top
    let e4 = effect::create_effect(d, c);
    effect::set_category(d, e4, CATEGORY_TODECK);
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e4, LOCATION_FZONE);
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    effect::set_target(d, e4, Some(cost_target(tdtg)));
    effect::set_operation(d, e4, Some(operation(tdop)));
    card::register_effect(d, c, e4, false);
}
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_ADAMANCIPATOR as u16])
}
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let g = duel::get_matching_group(d, Some(&setfilter), a.tp, LOCATION_DECK, 0, NO_EX);
        return matches!(
            aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(1), Some(5), None, 0, a.tp, None, None, None, false),
            Err(true)
        );
    }
    true
}
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let g = duel::get_matching_group(d, Some(&setfilter), tp, LOCATION_DECK, 0, NO_EX);
    let rg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(1), Some(5), None, 1, tp, Some(aux::stringid(ID, 1)), None, None, false).unwrap_or_default();
    if !rg.is_empty() {
        duel::confirm_cards(d, 1 - tp, rg.clone());
        duel::shuffle_deck(d, tp);
        duel::move_to_deck_top(d, rg.clone());
        duel::sort_decktop(d, tp, tp, rg.len() as u16);
    }
}
