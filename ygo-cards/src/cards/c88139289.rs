//! Psychic Blaster Mk-II
use super::prelude::*;

pub const ID: u32 = 88139289;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: 1 Psychic Tuner + 1+ non-Tuner Psychic monsters
    synchro::add_procedure(
        d,
        c,
        Some(crate::procs::mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_PSYCHIC, sc, st, Some(tp)))),
        1,
        1,
        Some(synchro::non_tuner_ex(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_PSYCHIC, sc, st, Some(tp)))),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // Special Summon 1 banished monster to your field
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Banish 1 other monster on the field then gain LP equal to its original ATK
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_REMOVE + CATEGORY_RECOVER);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    effect::set_cost(d, e2, Some(cost_target(rmvcost)));
    effect::set_target(d, e2, Some(cost_target(rmvtg)));
    effect::set_operation(d, e2, Some(operation(rmvop)));
    card::register_effect(d, c, e2, false);
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_faceup(d, c) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_REMOVED) && spfilter(d, chkc, e, tp);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_REMOVED, LOCATION_REMOVED, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_REMOVED, LOCATION_REMOVED, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_monster(d, c) && card::is_able_to_remove_as_cost(d, c, None)
}
fn rmvcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&costfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&costfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
fn rmvtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc) && card::is_able_to_remove(d, chkc, None, None, None) && chkc != c;
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| card::is_faceup(d, x) && card::is_able_to_remove(d, x, None, None, None)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, c.into());
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    let sel = duel::select_target(d, tp, Some(&|d, x| card::is_faceup(d, x) && card::is_able_to_remove(d, x, None, None, None)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, c.into()).nil();
    let tc = group::get_first(d, &sel).nil();
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(tc)), 1, tp, 0);
    let atk = card::get_base_attack(d, tc);
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 1, tp, atk);
    true
}
fn rmvop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) && duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0 && card::is_location(d, tc, LOCATION_REMOVED) {
        let atk = card::get_base_attack(d, tc);
        if atk > 0 {
            duel::break_effect(d);
            duel::recover(d, tp, atk as i64, REASON_EFFECT, false, None);
        }
    }
}
