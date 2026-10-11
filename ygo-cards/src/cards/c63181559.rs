//! Trickstar Diffusion
//! scripted by Naim
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 63181559;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TRICKSTAR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(efftg)));
    effect::set_operation(d, e1, Some(operation(effop)));
    card::register_effect(d, c, e1, false);
    // This turn, your opponent's monsters can only attack the targeted monster while it is face-up
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_hint_timing(d, e2, 0, Some(TIMING_BATTLE_START));
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(atklimcond)));
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(atklimtg)));
    effect::set_operation(d, e2, Some(operation(atklimop)));
    card::register_effect(d, c, e2, false);
}
fn fusion_params() -> fusion::SummonEff {
    fusion::SummonEff {
        fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_set_card(d, c, &[SET_TRICKSTAR as u16]))),
        matfilter: Some(fusion::mat_filter_fn(|_d, _c, _e, _tp, _chk| false)),
        extrafil: Some(Arc::new(|d: &mut Duel, e: EffectId, tp: u8, mg: &CardSet| fextra(d, e, tp, mg))),
        extraop: Some(fusion::banish_material()),
        ..Default::default()
    }
}
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    let location = if duel::is_player_affected_by_effect(d, tp, CARD_SPIRIT_ELIMINATION).is_none() { LOCATION_GRAVE } else { LOCATION_MZONE };
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None)), tp, location, 0, NO_EX);
    (Some(g), None)
}
fn linkfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_link_summonable(d, c, None, None, 0, 0) && card::is_set_card(d, c, &[SET_TRICKSTAR as u16])
}
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let p = fusion_params();
    let mut a0 = a.clone();
    a0.chk = Some(0);
    let b1 = fusion::summon_eff_tg_call(d, &p, &a0);
    let b2 = duel::is_existing_matching_card(d, Some(&linkfilter), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    if a.chk == Some(0) {
        return b1 || b2;
    }
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]).nil() as i64;
    effect::set_label(d, a.e, &[op]);
    if op == 1 {
        effect::set_category(d, a.e, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, LOCATION_GRAVE as i32);
    } else if op == 2 {
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
fn effop(d: &mut Duel, a: &EffArgs) {
    let op = effect::get_label(d, a.e);
    if op == 1 {
        // Fusion Summon 1 "Trickstar" Fusion Monster by banishing its materials from your GY
        let p = fusion_params();
        fusion::summon_eff_op_call(d, &p, a);
    } else if op == 2 {
        // Link Summon 1 "Trickstar" Link Monster
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        let sel = duel::select_matching_card(d, a.tp, Some(&linkfilter), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        if let Some(sc) = group::get_first(d, &sel) {
            duel::link_summon(d, a.tp, sc, None, None, 0, 0);
        }
    }
}
fn atklimfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_TRICKSTAR as u16])
}
fn atklimcond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp) && duel::get_current_phase(d) <= PHASE_BATTLE
}
fn atklimtg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::get_controler(d, chkc) == a.tp && card::is_faceup(d, chkc) && card::is_set_card(d, chkc, &[SET_TRICKSTAR as u16]);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&atklimfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    duel::select_target(d, a.tp, Some(&atklimfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
fn atklimop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        let c = effect::get_handler(d, a.e).nil();
        let fid = card::get_field_id(d, tc);
        card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 4));
        // Your opponent's monsters cannot target monsters for attacks, except that monster
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        effect::set_code(d, e1, EFFECT_ONLY_ATTACK_MONSTER);
        effect::set_target_range(d, e1, 0, LOCATION_MZONE);
        effect::set_condition(d, e1, Some(condition(move |d, _a| card::has_flag_effect(d, tc, ID, None) && card::get_field_id(d, tc) == fid)));
        effect::set_value_fn(d, e1, effect_card_fn(move |d, _e, c| card::has_flag_effect(d, c, ID, None) && card::get_field_id(d, c) == fid));
        effect::set_reset(d, e1, RESET_PHASE + PHASE_END, 1);
        duel::register_effect(d, e1, a.tp);
        aux::register_client_hint(d, c, 0, a.tp, 0, 1, Some(aux::stringid(ID, 5)), 0, None);
    }
}
