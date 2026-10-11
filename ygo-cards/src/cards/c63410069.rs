//! Desert Locusts
use super::prelude::*;

pub const ID: u32 = 63410069;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Must be properly summoned before reviving
    card::enable_revive_limit(d, c);
    // Synchro summon procedure
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // If synchro summoned, the turn player discards 1
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_HANDES);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(tgcon)));
    effect::set_target(d, e1, Some(cost_target(tgtg)));
    effect::set_operation(d, e1, Some(operation(tgop)));
    card::register_effect(d, c, e1, false);
    // Synchro summon by effect
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_MAIN_END));
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_condition(d, e2, Some(condition(sccon)));
    effect::set_target(d, e2, Some(cost_target(sctarg)));
    effect::set_operation(d, e2, Some(operation(scop)));
    card::register_effect(d, c, e2, false);
}
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_synchro_summoned(d, c)
}
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let cp = duel::get_turn_player(d);
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, cp, 1);
    true
}
fn tgop(d: &mut Duel, _a: &EffArgs) {
    let cp = duel::get_turn_player(d);
    let g = duel::get_field_group(d, cp, LOCATION_HAND, 0);
    if g.len() == 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, cp, HINTMSG_DISCARD as u64);
    let sg = group::select(d, &g, cp, 1, 1, false, NO_EX).nil();
    duel::send_to_grave(d, sg, REASON_DISCARD | REASON_EFFECT, None, None);
}
fn sccon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp) && duel::is_main_phase(d, None)
}
fn sctarg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::get_flag_effect(d, c, ID) == 0
            && duel::is_existing_matching_card(d, Some(&|d, x| card::is_synchro_summonable(d, x, Some(CardSet::from_card(c)), None, 0, 0)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    card::register_flag_effect(d, c, ID, RESET_CHAIN, 0, 1, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
fn scop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_controler(d, c, 1 - a.tp) || !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) {
        return;
    }
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_synchro_summonable(d, x, Some(CardSet::from_card(c)), None, 0, 0)), a.tp, LOCATION_EXTRA, 0, NO_EX);
    if g.len() > 0 {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sg).nil();
        duel::synchro_summon(d, a.tp, sc, Some(CardSet::from_card(c)), None, 0, 0);
    }
}
