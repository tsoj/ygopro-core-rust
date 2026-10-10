//! Shooting Riser Dragon
use super::prelude::*;

pub const ID: u32 = 68431965;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Synchro Summon procedure
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Send to the GY and decrease Level
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOGRAVE);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(lvlcon)));
    effect::set_target(d, e1, Some(cost_target(lvtg)));
    effect::set_operation(d, e1, Some(operation(lvop)));
    card::register_effect(d, c, e1, false);
    // Synchro Summon during the opponent's Main Phase
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_MAIN_END));
    effect::set_condition(d, e2, Some(condition(sccon)));
    effect::set_target(d, e2, Some(cost_target(sctarg)));
    effect::set_operation(d, e2, Some(operation(scop)));
    card::register_effect(d, c, e2, false);
}
fn lvlcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_synchro_summoned(d, c)
}
fn tgfilter(d: &mut Duel, c: CardId, lv: u32) -> bool {
    lv >= 1 && card::is_level_below(d, c, lv - 1) && card::is_able_to_grave(d, c, None)
}
fn lvtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let lv = card::get_level(d, effect::get_handler(d, a.e).nil());
        return duel::is_existing_matching_card(d, Some(&|d, c| tgfilter(d, c, lv)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn aclimit(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    let rh = effect::get_handler(d, re).nil();
    card::is_code(d, rh, &[effect::get_label(d, a.e(0)) as u32]) && effect::is_monster_effect(d, re)
}
fn lvop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !(card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c)) {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    let lv = card::get_level(d, c);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| tgfilter(d, x, lv)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if g.is_empty() {
        return;
    }
    let sent = duel::send_to_grave(d, g.clone(), REASON_EFFECT, None, None);
    let in_grave = group::filter_count(d, &g, &|d, x| card::is_location(d, x, LOCATION_GRAVE), NO_EX) as i32;
    if sent != in_grave {
        return;
    }
    let tc = group::get_first(d, &g).nil();
    let lv = card::get_level(d, tc);
    // Change Level
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_value(d, e1, -(lv as i64));
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, c, e1, false);
    // Cannot activate the effects of monsters with the same name
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e2, EFFECT_CANNOT_ACTIVATE);
    effect::set_target_range(d, e2, 1, 0);
    effect::set_value_fn(d, e2, lua_fn(aclimit));
    let code = card::get_code(d, tc)[0] as i64;
    effect::set_label(d, e2, &[code]);
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e2, a.tp);
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e3, EFFECT_CANNOT_TRIGGER);
    effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
    card::register_effect(d, tc, e3, false);
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
    if !g.is_empty() {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sg).nil();
        duel::synchro_summon(d, a.tp, sc, Some(CardSet::from_card(c)), None, 0, 0);
    }
}
