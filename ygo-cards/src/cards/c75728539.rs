//! Sharv Sarga
use super::prelude::*;

pub const ID: u32 = 75728539;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_VISAS_STARFROST], listed_series: &[SET_VEDA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Shuffle 1 Effect Monster into the Deck
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(tdcon)));
    effect::set_target(d, e1, Some(cost_target(tdtg)));
    effect::set_operation(d, e1, Some(operation(tdop)));
    card::register_effect(d, c, e1, false);
    // Add this card to the hand if a "Veda" monster is Special Summoned
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOHAND);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(thcon)));
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
}
fn tdcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[CARD_VISAS_STARFROST])), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX)
}
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_EFFECT) && card::is_able_to_deck(d, c)
}
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && tdfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tdfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    let g = duel::select_target(d, a.tp, Some(&tdfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_TODECK, g.clone(), 1, 0, 0);
    let first = group::get_first(d, &g.clone().nil()).nil();
    let owner = card::get_owner(d, first);
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, owner, LOCATION_DECK as i32);
    true
}
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) && tdfilter(d, tc) && duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0 && card::is_location(d, tc, LOCATION_DECK | LOCATION_EXTRA) {
        let rac = card::get_race(d, tc);
        let lv = card::get_level(d, tc) as i64;
        let rk = card::get_rank(d, tc) as i64;
        let lk = card::get_link(d, tc) as i64;
        let val = lv.max(rk).max(lk);
        // The owner can search a monster with different type and lower Level
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        effect::set_description(d, e1, aux::stringid(ID, 1));
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        effect::set_label(d, e1, &[rac as i64, val]);
        effect::set_condition(d, e1, Some(condition(srchcon)));
        effect::set_operation(d, e1, Some(operation(srchop)));
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        let owner = card::get_owner(d, tc);
        duel::register_effect(d, e1, owner);
    }
}
fn srcfilter(d: &mut Duel, c: CardId, rac: u64, lv: i64) -> bool {
    card::is_monster(d, c) && card::is_able_to_hand(d, c, None) && !card::is_race(d, c, rac) && (card::get_level(d, c) as i64) < lv
}
fn srchcon(d: &mut Duel, a: &EffArgs) -> bool {
    let labels = effect::get_labels(d, a.e);
    let rac = labels.first().copied().unwrap_or(0) as u64;
    let lv = labels.get(1).copied().unwrap_or(0);
    duel::is_existing_matching_card(d, Some(&|d, c| srcfilter(d, c, rac, lv)), a.tp, LOCATION_DECK, 0, 1, NO_EX)
}
fn srchop(d: &mut Duel, a: &EffArgs) {
    if !duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
        return;
    }
    duel::hint(d, HINT_CARD as u8, 1 - a.tp, ID as u64);
    let labels = effect::get_labels(d, a.e);
    let rac = labels.first().copied().unwrap_or(0) as u64;
    let lv = labels.get(1).copied().unwrap_or(0);
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| srcfilter(d, c, rac, lv)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    group::is_exists(d, &a.eg.clone().nil(), &|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_VEDA as u16]), 1, NO_EX)
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
        duel::confirm_cards(d, 1 - a.tp, c);
    }
}
