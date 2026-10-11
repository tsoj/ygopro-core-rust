//! Veda Kalarcanum
use super::prelude::*;

pub const ID: u32 = 40785230;
const COUNTER_VEDA: u16 = 0x210;

// s.listed_counter={COUNTER_VEDA}: nothing reads it, left out
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VEDA], listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    card::enable_counter_permit(d, c, COUNTER_VEDA, Some(LOCATION_PZONE), None);
    // Can only be Special Summoned once per turn
    card::set_spsummon_once(d, c, ID);
    // Pendulum attributes
    pendulum::add_procedure(d, c, None, None);
    // Special Summon condition
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e0, EFFECT_SPSUMMON_CONDITION);
    card::register_effect(d, c, e0, false);
    // Place 1 Counter when a monster is destroyed
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_COUNTER);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_DESTROYED);
    effect::set_range(d, e1, LOCATION_PZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, EFFECT_COUNT_CODE_CHAIN as u8);
    effect::set_condition(d, e1, Some(condition(ctcon)));
    effect::set_target(d, e1, Some(cost_target(cttg)));
    effect::set_operation(
        d,
        e1,
        Some(operation(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            card::add_counter(d, h, COUNTER_VEDA, 3, false);
        })),
    );
    card::register_effect(d, c, e1, false);
    // Increase Scale by the number of counters on itself
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e2, EFFECT_UPDATE_LSCALE);
    effect::set_range(d, e2, LOCATION_PZONE);
    effect::set_value_fn(d, e2, effect_card_value(scale_value));
    card::register_effect(d, c, e2, false);
    let e2b = effect::clone(d, e2);
    effect::set_code(d, e2b, EFFECT_UPDATE_RSCALE);
    card::register_effect(d, c, e2b, false);
    // Special Summon itself from the Pendulum Zone
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e3, LOCATION_PZONE);
    effect::set_cost(d, e3, Some(cost_target(pspcost)));
    effect::set_target(d, e3, Some(cost_target(psptg)));
    effect::set_operation(d, e3, Some(operation(pspop)));
    card::register_effect(d, c, e3, false);
    // Make it become the End Phase
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 2));
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e4, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    effect::set_condition(d, e4, Some(condition(epcon)));
    effect::set_cost(d, e4, Some(cost_target(epcost)));
    effect::set_operation(d, e4, Some(operation(epop)));
    card::register_effect(d, c, e4, false);
    // Return itself to the hand and Special Summon 1 "Veda" monster
    let e5 = effect::create_effect(d, c);
    effect::set_description(d, e5, aux::stringid(ID, 3));
    effect::set_category(d, e5, CATEGORY_TOHAND | CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e5, EVENT_PHASE | PHASE_STANDBY);
    effect::set_range(d, e5, LOCATION_MZONE);
    effect::set_count_limit(d, e5, 1, 0, 0, 0);
    effect::set_condition(d, e5, Some(condition(|d, a| duel::is_turn_player(d, a.tp))));
    effect::set_target(d, e5, Some(cost_target(thtg)));
    effect::set_operation(d, e5, Some(operation(thop)));
    card::register_effect(d, c, e5, false);
}
fn scale_value(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    let h = effect::get_handler(d, e).nil();
    card::get_counter(d, h, COUNTER_VEDA) as i64
}
fn ctcfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_previous_location(d, c, LOCATION_MZONE) || (!card::is_previous_location(d, c, LOCATION_ONFIELD) && card::is_monster(d, c))
}
fn ctcon(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &ctcfilter, 1, NO_EX)
}
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_can_add_counter(d, h, COUNTER_VEDA, 3, false, 0);
    }
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, None, 3, a.tp, COUNTER_VEDA as i32);
    true
}
fn pspcost(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_can_remove_counter(d, c, a.tp, COUNTER_VEDA, 12, REASON_COST);
    }
    card::remove_counter(d, c, a.tp, COUNTER_VEDA, 12, REASON_COST);
    true
}
fn psptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, true, true, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn pspop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon(d, c, 0, a.tp, a.tp, true, true, POS_FACEUP as u8, None) > 0 {
        card::complete_procedure(d, c);
    }
}
fn epcfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_summon_player(d, c, 1 - tp) && card::is_summon_location(d, c, LOCATION_EXTRA)
}
fn epcon(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| epcfilter(d, c, a.tp), 1, NO_EX) && duel::get_current_phase(d) != PHASE_END
}
fn epcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let loc = LOCATION_HAND | LOCATION_ONFIELD | LOCATION_GRAVE;
    let f = |d: &mut Duel, c: CardId| card::is_able_to_remove_as_cost(d, c, Some(POS_FACEDOWN as u8));
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&f), tp, loc, 0, 12, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&f), tp, loc, 0, 12, 12, false, NO_EX).nil();
    duel::remove(d, g, POS_FACEDOWN as u8, REASON_COST, None, None);
    true
}
fn epop(d: &mut Duel, a: &EffArgs) {
    let turn_player = duel::get_turn_player(d);
    duel::skip_phase(d, turn_player, PHASE_DRAW, RESET_PHASE | PHASE_END, 1, 0);
    duel::skip_phase(d, turn_player, PHASE_STANDBY, RESET_PHASE | PHASE_END, 1, 0);
    duel::skip_phase(d, turn_player, PHASE_MAIN1, RESET_PHASE | PHASE_END, 1, 0);
    duel::skip_phase(d, turn_player, PHASE_BATTLE, RESET_PHASE | PHASE_END, 1, 1);
    duel::skip_phase(d, turn_player, PHASE_MAIN2, RESET_PHASE | PHASE_END, 1, 0);
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CANNOT_BP);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, turn_player);
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(h)), 1, 0, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    true
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_VEDA as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && (card::is_faceup(d, c) || !card::is_location(d, c, LOCATION_REMOVED))
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if !(card::is_relate_to_effect(d, c, e)
        && duel::send_to_hand(d, c, None, REASON_EFFECT, None) > 0
        && card::is_location(d, c, LOCATION_HAND)
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0)
    {
        return;
    }
    let f = aux::necro_valley_filter(move |d, x| spfilter(d, x, e, tp));
    let g = duel::get_matching_group(d, Some(&f), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE | LOCATION_REMOVED, 0, NO_EX);
    if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 4)) {
        duel::shuffle_hand(d, tp);
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        duel::break_effect(d);
        duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
