//! Nibiru, the Primal Being
use super::prelude::*;

pub const ID: u32 = 27204311;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[27204312], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Tribute all monsters, Special Summon this card from the hand and 1 "Primal Being Token"
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_RELEASE + CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMING_SUMMON | TIMING_SPSUMMON));
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
    // Count summons
    aux::global_check(d, ID, |d| {
        let ge1 = effect::create_effect(d, c);
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, ge1, EVENT_SPSUMMON_SUCCESS);
        effect::set_operation(d, ge1, Some(operation(checkop)));
        duel::register_effect(d, ge1, 0);
        let ge2 = effect::clone(d, ge1);
        effect::set_code(d, ge2, EVENT_SUMMON_SUCCESS);
        duel::register_effect(d, ge2, 0);
    });
}
fn checkop(d: &mut Duel, a: &EffArgs) {
    let eg = a.eg.clone().nil();
    let _tc = group::get_first(d, &eg);
    for tc in group::iter(d, &eg) {
        let p = card::get_summon_player(d, tc);
        duel::register_flag_effect(d, p, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    }
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_flag_effect(d, 1 - a.tp, ID) >= 5 && duel::is_main_phase(d, None)
}
/// The Primal Being Token: `Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,atk,def,11,RACE_ROCK,ATTRIBUTE_LIGHT,POS_FACEUP,1-tp)`.
/// The Lua set code `0` is a number, so the core inserts it as a set code.
fn token_data(atk: i64, def: i64) -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(atk as i32),
        defense: Some(def as i32),
        level: Some(11),
        race: Some(RACE_ROCK),
        attribute: Some(ATTRIBUTE_LIGHT),
    }
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    let f = aux::faceup_filter(|d, c| card::is_releasable_by_effect(d, c, None, None));
    let g = duel::get_matching_group(d, Some(&f), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    if a.chk == Some(0) {
        let ok = !g.is_empty()
            && duel::get_m_zone_count(d, tp, (&g).into(), Some(tp), None, None).0 > 0
            && duel::get_m_zone_count(d, 1 - tp, (&g).into(), Some(tp), None, None).0 > 0
            && duel::is_player_can_special_summon_count(d, tp, 2)
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
        if !ok {
            return false;
        }
        let atk = group::get_sum(d, &g, &|d, c| card::get_base_attack(d, c) as i64);
        let def = group::get_sum(d, &g, &|d, c| card::get_base_defense(d, c) as i64);
        return duel::is_player_can_special_summon_monster(d, tp, ID + 1, token_data(atk, def), Some(POS_FACEUP as u8), Some(1 - tp), 0);
    }
    duel::set_operation_info(d, 0, CATEGORY_RELEASE, Some(g.clone()), g.len() as u8, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    true
}
fn operationvalueatk(d: &mut Duel, c: CardId) -> i64 {
    card::get_base_attack(d, c).max(0) as i64
}
fn operationvaluedef(d: &mut Duel, c: CardId) -> i64 {
    card::get_base_defense(d, c).max(0) as i64
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    let f = aux::faceup_filter(|d, c| card::is_releasable_by_effect(d, c, None, None));
    let g = duel::get_matching_group(d, Some(&f), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    if duel::release(d, g, REASON_EFFECT, None) == 0 {
        return;
    }
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let og = duel::get_operated_group(d);
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0
        && duel::get_m_zone_count(d, 1 - tp, NO_EX, Some(tp), None, None).0 > 0
    {
        let atk = group::get_sum(d, &og, &operationvalueatk);
        let def = group::get_sum(d, &og, &operationvaluedef);
        if !duel::is_player_can_special_summon_monster(d, tp, ID + 1, token_data(atk, def), Some(POS_FACEUP as u8), Some(1 - tp), 0) {
            return;
        }
        duel::break_effect(d);
        let token = duel::create_token(d, tp, ID + 1).nil();
        duel::special_summon_step(d, token, 0, tp, 1 - tp, false, false, POS_FACEUP as u8, None);
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_SET_ATTACK);
        effect::set_value(d, e1, atk);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, token, e1, false);
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e2, EFFECT_SET_DEFENSE);
        effect::set_value(d, e2, def);
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, token, e2, false);
        duel::special_summon_complete(d);
    }
}
