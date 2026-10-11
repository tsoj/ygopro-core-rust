//! Sacred Beasts Released
use super::prelude::*;

pub const ID: u32 = 38776201;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SACRED_BEAST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 3 "Sacred Beast" monsters with different names from your Deck to your hand, then discard 2 cards
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_HANDES);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(th3tg)));
    effect::set_operation(d, e1, Some(operation(th3op)));
    card::register_effect(d, c, e1, false);
    // If this card is in your GY, except the turn it was sent there: You can banish it; add 1 Level 10 Pyro, Thunder, or Fiend monster that cannot be Normal Summoned/Set from your Deck to your hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(aux::exccon()));
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(th1tg)));
    effect::set_operation(d, e2, Some(operation(th1op)));
    card::register_effect(d, c, e2, false);
}
fn th3filter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_SACRED_BEAST as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
fn th3tg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let g = duel::get_matching_group(d, Some(&th3filter), a.tp, LOCATION_DECK, 0, NO_EX);
        return matches!(
            aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(3), Some(3), Some(&aux::dncheck()), 0, a.tp, None, None, None, false),
            Err(true)
        );
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 3, a.tp, LOCATION_DECK as i32);
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, a.tp, 2);
    true
}
fn th3op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let g = duel::get_matching_group(d, Some(&th3filter), tp, LOCATION_DECK, 0, NO_EX);
    if g.len() < 3 {
        return;
    }
    let sg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(3), Some(3), Some(&aux::dncheck()), 1, tp, Some(HINTMSG_ATOHAND as u64), None, None, false).unwrap_or_default();
    if sg.len() == 3 && duel::send_to_hand(d, &sg, None, REASON_EFFECT, None) > 0 {
        duel::confirm_cards(d, 1 - tp, &sg);
        duel::shuffle_hand(d, tp);
        duel::break_effect(d);
        duel::discard_hand(d, tp, None, 2, 2, REASON_EFFECT | REASON_DISCARD, NO_EX);
    }
}
fn th1filter(d: &mut Duel, c: CardId) -> bool {
    card::is_level(d, c, &[10])
        && card::is_race(d, c, RACE_PYRO | RACE_THUNDER | RACE_FIEND)
        && !card::is_summonable_card(d, c)
        && card::is_able_to_hand(d, c, None)
}
fn th1tg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&th1filter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn th1op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&th1filter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() && duel::send_to_hand(d, &g, None, REASON_EFFECT, None) > 0 {
        duel::confirm_cards(d, 1 - tp, &g);
    }
}
