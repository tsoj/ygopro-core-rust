//! Garden Rose Flora
use super::prelude::*;

pub const ID: u32 = 76524506;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[TOKEN_ROSE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // synchro summon
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // destroy + token
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(destg)));
    effect::set_operation(d, e1, Some(operation(desop)));
    card::register_effect(d, c, e1, false);
    // synchro effect
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
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_FZONE);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, a.tp, LOCATION_FZONE, LOCATION_FZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    let g = duel::select_target(d, a.tp, None, a.tp, LOCATION_FZONE, LOCATION_FZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        let p = card::get_controler(d, tc);
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_description(d, e1, aux::stringid(ID, 1));
        effect::set_target_range(d, e1, 1, 0);
        effect::set_target(d, e1, Some(effect_card_fn(splimit)));
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        duel::register_effect(d, e1, a.tp);
        // Clock Lizard check
        aux::add_temp_lizard_check(d, h, a.tp, Some(effect_card_fn(lizfilter)), None, None, None, None);
        if duel::destroy(d, tc, REASON_EFFECT, None, None) > 0
            && duel::get_location_count(d, p, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_special_summon_monster(d, a.tp, TOKEN_ROSE, token_data(), Some(POS_FACEUP_ATTACK as u8), Some(p), 0)
        {
            let token = duel::create_token(d, a.tp, TOKEN_ROSE).nil();
            duel::special_summon(d, token, 0, a.tp, p, false, false, POS_FACEUP_ATTACK as u8, None);
        }
    }
}
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(800),
        defense: Some(800),
        level: Some(2),
        race: Some(RACE_PLANT),
        attribute: Some(ATTRIBUTE_DARK),
    }
}
/// `s.splimit(e,c)`
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    !card::is_type(d, c, TYPE_SYNCHRO) && card::is_location(d, c, LOCATION_EXTRA)
}
/// `s.lizfilter(e,c)`
fn lizfilter(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    !card::is_original_type(d, c, TYPE_SYNCHRO)
}
fn sccon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp) && duel::is_main_phase(d, None)
}
fn sctarg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(
            d,
            Some(&|d, x| card::is_synchro_summonable(d, x, Some(CardSet::from_card(c)), None, 0, 0)),
            a.tp,
            LOCATION_EXTRA,
            0,
            1,
            NO_EX,
        ) && card::get_flag_effect(d, c, ID) == 0;
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
    let g = duel::get_matching_group(
        d,
        Some(&|d, x| card::is_synchro_summonable(d, x, Some(CardSet::from_card(c)), None, 0, 0)),
        a.tp,
        LOCATION_EXTRA,
        0,
        NO_EX,
    );
    if !g.is_empty() {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sg).nil();
        duel::synchro_summon(d, a.tp, sc, Some(CardSet::from_card(c)), None, 0, 0);
    }
}
