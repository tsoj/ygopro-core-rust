//! Enlilgirsu, the Orcust Mekk-Knight
use super::prelude::*;

pub const ID: u32 = 74820316;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ORCUST, SET_WORLD_LEGACY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 2+ monsters, including an "Orcust" Link Monster
    link::add_procedure(d, c, None, 2, Some(4), Some(group_check(matcheck)), None, None);
    // Add 1 of your banished "Orcust" or "World Legacy" cards to your hand
    let e1a = effect::create_effect(d, c);
    effect::set_description(d, e1a, aux::stringid(ID, 0));
    effect::set_category(d, e1a, CATEGORY_TOHAND + CATEGORY_TODECK + CATEGORY_CONTROL);
    effect::set_type(d, e1a, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e1a, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e1a, LOCATION_MZONE);
    effect::set_count_limit(d, e1a, 1, ID, 0, 0);
    effect::set_condition(d, e1a, Some(condition(|d, a| !babelthcon(d, a))));
    effect::set_target(d, e1a, Some(cost_target(thtg)));
    effect::set_operation(d, e1a, Some(operation(thop)));
    card::register_effect(d, c, e1a, false);
    // Quick version if "Orcustrated Babel" is applying
    let e1b = effect::clone(d, e1a);
    effect::set_type(d, e1b, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1b, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1b, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    effect::set_condition(d, e1b, Some(condition(babelthcon)));
    card::register_effect(d, c, e1b, false);
    // Send 1 card on the field to the GY
    let e2a = effect::create_effect(d, c);
    effect::set_description(d, e2a, aux::stringid(ID, 1));
    effect::set_category(d, e2a, CATEGORY_TOGRAVE);
    effect::set_type(d, e2a, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2a, LOCATION_GRAVE);
    effect::set_count_limit(d, e2a, 1, ID, 1, 0);
    effect::set_condition(d, e2a, Some(condition(tgcond)));
    effect::set_cost(d, e2a, Some(cost::self_banish()));
    effect::set_target(d, e2a, Some(cost_target(tgtg)));
    effect::set_operation(d, e2a, Some(operation(tgop)));
    card::register_effect(d, c, e2a, false);
    // Quick version if "Orcustrated Babel" is applying
    let e2b = effect::clone(d, e2a);
    effect::set_type(d, e2b, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2b, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e2b, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    effect::set_condition(d, e2b, Some(condition(babeltgcond)));
    card::register_effect(d, c, e2b, false);
}
fn matfilter(d: &mut Duel, c: CardId, lc: CardId, sumtype: u64, tp: u8) -> bool {
    card::is_set_card_ctx(d, c, &[SET_ORCUST as u16], Some(lc), sumtype, Some(tp))
        && card::is_type_ctx(d, c, TYPE_LINK, Some(lc), sumtype, Some(tp))
}
fn matcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    group::is_exists(d, g, &|d, c| matfilter(d, c, lc, sumtype, tp), 1, NO_EX)
}
fn babelthcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_player_affected_by_effect(d, a.tp, CARD_ORCUSTRATED_BABEL).is_some()
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_ORCUST as u16, SET_WORLD_LEGACY as u16]) && card::is_able_to_hand(d, c, None) && card::is_faceup(d, c)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_REMOVED) && card::is_controler(d, chkc, a.tp) && thfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), a.tp, LOCATION_REMOVED, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_target(d, a.tp, Some(&thfilter), a.tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, a.tp, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_HAND as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_CONTROL, None, 1, a.tp, 0);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::send_to_hand(d, tc, None, REASON_EFFECT, None) > 0
        && card::is_location(d, tc, LOCATION_HAND)
        && duel::is_existing_matching_card(d, Some(&card::is_able_to_deck), a.tp, LOCATION_HAND, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_controler_can_be_changed(d, c, false, Some(0xff)))), a.tp, 0, LOCATION_MZONE, 1, NO_EX)
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2))
    {
        duel::shuffle_hand(d, a.tp);
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
        let g = duel::select_matching_card(d, a.tp, Some(&card::is_able_to_deck), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        if g.is_empty() {
            return;
        }
        duel::break_effect(d);
        duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONTROL as u64);
        let sg = duel::select_matching_card(d, a.tp, Some(&aux::faceup_filter(|d, c| card::is_controler_can_be_changed(d, c, false, Some(0xff)))), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sg);
        if let Some(sc) = sc {
            duel::hint_selection(d, sc, true);
            duel::get_control(d, sc, a.tp, 0, 0, None, None);
        }
    }
}
fn tgcond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    duel::is_player_affected_by_effect(d, a.tp, CARD_ORCUSTRATED_BABEL).is_none()
        && card::is_previous_location(d, c, LOCATION_EMZONE)
        && card::get_turn_id(d, c) as i32 == duel::get_turn_count(d, None)
}
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(g), 1, a.tp, 0);
    true
}
fn tgop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, a.tp, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::hint_selection(d, g.clone(), true);
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
fn babeltgcond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    duel::is_player_affected_by_effect(d, a.tp, CARD_ORCUSTRATED_BABEL).is_some()
        && card::is_previous_location(d, c, LOCATION_EMZONE)
        && card::get_turn_id(d, c) as i32 == duel::get_turn_count(d, None)
}
