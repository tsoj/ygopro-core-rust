//! Megalith Anastasis
use super::prelude::*;

pub const ID: u32 = 29876299;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_MEGALITH],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e0, false);
    // Add 2 "Megalith" monsters (1 Level 4 or lower and 1 Level 8 or higher) from your Deck to your hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_SZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost::discard(None, false, None, None, None)));
    effect::set_target(d, e1, Some(cost_target(thtg)));
    effect::set_operation(d, e1, Some(operation(thop)));
    card::register_effect(d, c, e1, false);
    // Activate 1 of these effects
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(effcon)));
    effect::set_target(d, e2, Some(cost_target(efftg)));
    effect::set_operation(d, e2, Some(operation(effop)));
    card::register_effect(d, c, e2, false);
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    (card::is_level_below(d, c, 4) || card::is_level_above(d, c, 8))
        && card::is_set_card(d, c, &[SET_MEGALITH as u16])
        && card::is_able_to_hand(d, c, None)
}
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    let low = group::is_exists(d, sg, &|d, c| card::is_level_below(d, c, 4), 1, NO_EX);
    (low && group::is_exists(d, sg, &|d, c| card::is_level_above(d, c, 8), 1, NO_EX), false)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let g = duel::get_matching_group(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, NO_EX);
        return matches!(
            aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 0, a.tp, None, None, None, false),
            Err(true)
        );
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 2, a.tp, LOCATION_DECK as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let g = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
    if g.len() == 0 {
        return;
    }
    let sg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_ATOHAND as u64), None, None, false)
        .unwrap_or_default();
    if sg.len() > 0 {
        duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, &sg);
    }
}
fn effconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_summon_player(d, c, tp) && card::is_ritual_summoned(d, c) && card::is_set_card(d, c, &[SET_MEGALITH as u16]) && card::is_faceup(d, c)
}
fn effcon(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| effconfilter(d, c, a.tp), 1, NO_EX)
}
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let b1 = !duel::has_flag_effect(d, tp, ID, None) && duel::is_player_can_draw(d, tp, 2);
    let b2 = !duel::has_flag_effect(d, tp, ID + 1, None)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_releasable_by_effect(d, c, None, None)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    if a.chk == Some(0) {
        return b1 || b2;
    }
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]).map_or(0, |x| x as i64);
    effect::set_label(d, a.e, &[op]);
    if op == 1 {
        duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        effect::set_category(d, a.e, CATEGORY_DRAW | CATEGORY_HANDES);
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 1, tp, 2);
        duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 1, tp, 1);
    } else if op == 2 {
        duel::register_flag_effect(d, tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
        effect::set_category(d, a.e, CATEGORY_RELEASE);
        duel::set_operation_info(d, 0, CATEGORY_RELEASE, None, 1, 1 - tp, LOCATION_MZONE as i32);
    }
    true
}
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let op = effect::get_label(d, a.e);
    if op == 1 {
        // Draw 2 cards, then discard 1 card
        if duel::draw(d, tp, 2, REASON_EFFECT) == 2 {
            duel::shuffle_hand(d, tp);
            duel::break_effect(d);
            duel::discard_hand(d, tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
        }
    } else if op == 2 {
        // Tribute 1 monster your opponent controls
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RELEASE as u64);
        let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_releasable_by_effect(d, c, None, None)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        if g.len() > 0 {
            duel::hint_selection(d, g.clone(), true);
            duel::release(d, g, REASON_EFFECT, None);
        }
    }
}
