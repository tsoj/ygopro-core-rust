//! Ars Magna of Unification and Separation
use super::prelude::*;

pub const ID: u32 = 99753860;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ARS_MAGNA, SET_POWER_PATRON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You can banish this card from your hand; Special Summon 1 non-Spellcaster "Ars Magna" monster from your GY or banishment, also your "Power Patron" Link Monsters can make up to 3 attacks on monsters during each Battle Phase this turn
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost::self_banish()));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // If a Synchro and/or Link Monster(s) is Special Summoned while this card is banished: You can Special Summon this card
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
    // You can banish 1 "Ars Magna" card from your GY; draw 1 card
    let e3a = effect::create_effect(d, c);
    effect::set_description(d, e3a, aux::stringid(ID, 2));
    effect::set_category(d, e3a, CATEGORY_DRAW);
    effect::set_type(d, e3a, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e3a, LOCATION_MZONE);
    effect::set_count_limit(d, e3a, 1, ID, 2, 0);
    effect::set_condition(d, e3a, Some(condition(e3a_cond)));
    effect::set_cost(d, e3a, Some(cost_target(drawcost)));
    effect::set_target(d, e3a, Some(cost_target(drawtg)));
    effect::set_operation(d, e3a, Some(operation(drawop)));
    card::register_effect(d, c, e3a, false);
    // Quick version if the effect of "Ars Magna - "Citrinitas"" is applying
    let e3b = effect::clone(d, e3a);
    effect::set_type(d, e3b, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e3b, EVENT_FREE_CHAIN);
    effect::set_condition(d, e3b, Some(condition(citrinitascon)));
    effect::set_hint_timing(d, e3b, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    card::register_effect(d, c, e3b, false);
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_ARS_MAGNA as u16]) && card::is_race_except(d, c, RACE_SPELLCASTER as u64, None, 0, None) && card::is_faceup(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        let f = aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp));
        let g = duel::select_matching_card(d, a.tp, Some(&f), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
        if !g.is_empty() {
            duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
    let c = effect::get_handler(d, a.e).nil();
    aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 3)), 0, None);
    // Also your "Power Patron" Link Monsters can make up to 3 attacks on monsters during each Battle Phase this turn
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_EXTRA_ATTACK_MONSTER);
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_POWER_PATRON as u16]) && card::is_link_monster(d, c))));
    effect::set_value(d, e1, 2);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
}
fn citrinitascon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_player_affected_by_effect(d, a.tp, EFFECT_ARS_MAGNA_CITRINITAS).is_some()
}
fn drawcostfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_ARS_MAGNA as u16]) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, true, false)
}
fn drawcost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&drawcostfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&drawcostfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| card::is_faceup(d, c) && card::is_type(d, c, TYPE_SYNCHRO | TYPE_LINK), 1, NO_EX)
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
    !citrinitascon(d, a)
}
fn drawtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1);
    }
    duel::set_target_player(d, a.tp);
    duel::set_target_param(d, 1);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
fn drawop(d: &mut Duel, _a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
