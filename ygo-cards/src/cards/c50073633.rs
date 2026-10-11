//! Celtic Mystic
use super::prelude::*;

pub const ID: u32 = 50073633;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_LIGHT_AND_DARKNESS_RITUAL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If this card is Normal or Special Summoned: draw/discard
    let e1a = effect::create_effect(d, c);
    effect::set_description(d, e1a, aux::stringid(ID, 0));
    effect::set_category(d, e1a, CATEGORY_DRAW + CATEGORY_HANDES);
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1a, EFFECT_FLAG_DELAY + EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1a, EVENT_SUMMON_SUCCESS);
    effect::set_count_limit(d, e1a, 1, ID, 0, 0);
    effect::set_target(d, e1a, Some(cost_target(drtg)));
    effect::set_operation(d, e1a, Some(operation(drop_op)));
    card::register_effect(d, c, e1a, false);
    let e1b = effect::clone(d, e1a);
    effect::set_code(d, e1b, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e1b, false);
    // Tribute: Special Summon 1 Ritual Monster from hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_cost(d, e2, Some(cost::self_tribute()));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
}
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return !duel::is_existing_matching_card(d, Some(&|d, c| card::is_public(d, c)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::set_possible_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 3);
    duel::set_possible_operation_info(d, 0, CATEGORY_HANDES, None, 0, a.tp, 2);
    true
}
fn drop_op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    aux::register_client_hint(d, c, 0, tp, 1, 0, Some(aux::stringid(ID, 2)), 0, None);
    // You can only Special Summon from the Extra Deck once for the rest of this turn
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_condition(d, e1, Some(condition(sprestrictioncon)));
    effect::set_operation(d, e1, Some(operation(sprestrictionop)));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, tp);
    if duel::is_existing_matching_card(d, Some(&|d, c| card::is_public(d, c)), tp, LOCATION_HAND, 0, 1, NO_EX) {
        return;
    }
    let g = duel::get_field_group(d, tp, LOCATION_HAND, 0);
    duel::confirm_cards(d, 1 - tp, &g);
    duel::shuffle_hand(d, tp);
    let go = group::is_exists(d, &g, &|d, c| card::lists_code(d, c, &[CARD_LIGHT_AND_DARKNESS_RITUAL]), 1, NO_EX)
        && duel::is_player_can_draw(d, tp, 3)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 3))
        && duel::draw(d, tp, 3, REASON_EFFECT) == 3;
    if go {
        duel::shuffle_hand(d, tp);
        duel::break_effect(d);
        duel::discard_hand(d, tp, None, 2, 2, REASON_EFFECT | REASON_DISCARD, NO_EX);
    }
}
fn sprestrictionconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_summon_player(d, c, tp) && card::is_summon_location(d, c, LOCATION_EXTRA)
}
fn sprestrictioncon(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| sprestrictionconfilter(d, c, a.tp), 1, NO_EX)
}
fn sprestrictionop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA))));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
    // "Clock Lizard" check
    aux::add_temp_lizard_check(d, c, a.tp, None, None, None, None, None);
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_ritual_monster(d, c) && card::lists_code(d, c, &[CARD_LIGHT_AND_DARKNESS_RITUAL]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, true, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return duel::get_m_zone_count(d, a.tp, c.into(), None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::special_summon(d, &g, 0, tp, tp, false, true, POS_FACEUP as u8, None);
    }
}
