//! Ars Magna of Infinity and Finity
use super::prelude::*;

pub const ID: u32 = 36270527;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ARS_MAGNA, SET_POWER_PATRON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You can banish this card from your hand; add 1 non-Warrior "Ars Magna" monster from your Deck to your hand, also the original ATK of "Power Patron" Link Monsters you control become tripled for the rest of this turn
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost::self_banish()));
    effect::set_target(d, e1, Some(cost_target(thatktg)));
    effect::set_operation(d, e1, Some(operation(thatkop)));
    card::register_effect(d, c, e1, false);
    // If a Fusion and/or Link Monster(s) is Special Summoned while this card is banished: You can Special Summon this card
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_range(d, e2, LOCATION_REMOVED);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    effect::set_target(d, e2, Some(cost_target(e2_target)));
    effect::set_operation(d, e2, Some(operation(e2_operation)));
    card::register_effect(d, c, e2, false);
    // If you control a "Power Patron" Link Monster: You can target 1 monster on the field; banish it
    let e3a = effect::create_effect(d, c);
    effect::set_description(d, e3a, aux::stringid(ID, 2));
    effect::set_category(d, e3a, CATEGORY_REMOVE);
    effect::set_type(d, e3a, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e3a, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e3a, LOCATION_MZONE);
    effect::set_count_limit(d, e3a, 1, ID, 2, 0);
    effect::set_condition(d, e3a, Some(condition(e3a_cond)));
    effect::set_target(d, e3a, Some(cost_target(bantg)));
    effect::set_operation(d, e3a, Some(operation(banop)));
    card::register_effect(d, c, e3a, false);
    // Quick version if the effect of "Ars Magna - "Citrinitas"" is applying
    let e3b = effect::clone(d, e3a);
    effect::set_type(d, e3b, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e3b, EVENT_FREE_CHAIN);
    effect::set_condition(d, e3b, Some(condition(e3b_cond)));
    effect::set_hint_timing(d, e3b, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    card::register_effect(d, c, e3b, false);
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_ARS_MAGNA as u16]) && card::is_monster(d, c) && card::is_race_except(d, c, RACE_WARRIOR as u64, None, 0, None) && card::is_able_to_hand(d, c, None)
}
fn thatktg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_ATKCHANGE, None, 1, a.tp, 0);
    true
}
fn thatkop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - a.tp, g);
    }
    let c = effect::get_handler(d, a.e).nil();
    aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 3)), 0, None);
    // Also the original ATK of "Power Patron" Link Monsters you control become tripled for the rest of this turn
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_SET_BASE_ATTACK);
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_POWER_PATRON as u16]) && card::is_link_monster(d, c))));
    effect::set_value_fn(d, e1, effect_card_value(|d, _e, c| card::get_base_attack(d, c) as i64 * 3));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
}
fn citrinitascon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_player_affected_by_effect(d, a.tp, EFFECT_ARS_MAGNA_CITRINITAS).is_some()
}
fn powerpatronfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_POWER_PATRON as u16]) && card::is_link_monster(d, c) && card::is_faceup(d, c)
}
fn bancon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&powerpatronfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| card::is_faceup(d, c) && card::is_type(d, c, TYPE_FUSION | TYPE_LINK), 1, NO_EX)
}
fn e2_target(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
fn e2_operation(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
fn e3a_cond(d: &mut Duel, a: &EffArgs) -> bool {
    bancon(d, a) && !citrinitascon(d, a)
}
fn e3b_cond(d: &mut Duel, a: &EffArgs) -> bool {
    bancon(d, a) && citrinitascon(d, a)
}
fn bantg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_able_to_remove(d, chkc, None, None, None);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), 1, a.tp, 0);
    true
}
fn banop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
