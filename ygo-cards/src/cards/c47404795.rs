//! Abyss Actor - Super Producer
use super::prelude::*;

pub const ID: u32 = 47404795;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[77297908], listed_series: &[SET_ABYSS_ACTOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Link Summon procedure
    link::add_procedure(d, c, None, 2, Some(2), Some(group_check(|d, g, lc, st, tp| lcheck(d, g, lc, st, tp))), None, None);
    // Destroy 1 face-up card
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_MAIN_END));
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    effect::set_target(d, e1, Some(cost_target(destg)));
    effect::set_operation(d, e1, Some(operation(desop)));
    card::register_effect(d, c, e1, false);
}
/// `s.lcheck(g,lc,sumtype,tp)`
fn lcheck(d: &mut Duel, g: &CardSet, lc: CardId, st: u64, tp: u8) -> bool {
    group::is_exists(d, g, &|d, c| card::is_race_ctx(d, c, RACE_FIEND, Some(lc), st, Some(tp)), 1, NO_EX)
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, tp) && card::is_faceup(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
fn fldfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, &[77297908]) && card::is_field_spell(d, c) && !card::is_forbidden(d, c)
}
fn pendfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_ABYSS_ACTOR as u16]) && card::is_type(d, c, TYPE_PENDULUM) && !card::is_forbidden(d, c)
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let tc = duel::first_target(d).nil();
    if !card::is_relate_to_effect(d, tc, a.e) || duel::destroy(d, tc, REASON_EFFECT, None, None) == 0 {
        return;
    }
    let g1 = duel::get_matching_group(d, Some(&fldfilter), tp, LOCATION_DECK, 0, NO_EX);
    let g2 = duel::get_matching_group(d, Some(&pendfilter), tp, LOCATION_DECK, 0, NO_EX);
    let b1 = !g1.is_empty();
    let b2 = !g2.is_empty() && duel::check_pendulum_zones(d, tp);
    if !((b1 || b2) && duel::select_yes_no(d, tp, aux::stringid(ID, 1))) {
        return;
    }
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]);
    if op == Some(1) {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
        let tc = duel::select_matching_card(d, tp, Some(&fldfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        let Some(tc) = group::get_first(d, &tc) else { return };
        let fc = duel::get_field_card(d, tp, LOCATION_FZONE, 0);
        if let Some(fc) = fc {
            duel::send_to_grave(d, fc, REASON_RULE, None, None);
        }
        duel::break_effect(d);
        duel::move_to_field(d, tc, tp, tp, LOCATION_FZONE as u16, POS_FACEUP as u8, true, None);
    } else if op == Some(2) {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
        let tc = duel::select_matching_card(d, tp, Some(&pendfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        let Some(tc) = group::get_first(d, &tc) else { return };
        duel::break_effect(d);
        duel::move_to_field(d, tc, tp, tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
    }
}
