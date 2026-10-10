//! Crusadia Vanguard
use super::prelude::*;

pub const ID: u32 = 55312487;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CRUSADIA as u32, SET_WORLD_LEGACY as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When you activate this card: You can also Tribute 1 "Crusadia" or "World Legacy" monster; if you did, Special Summon 1 "Crusadia" or "World Legacy" monster, with a different original name, from your Deck or GY
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMING_BATTLE_START | TIMINGS_CHECK_MONSTER_E));
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // While you control a "Crusadia" Link Monster, your opponent's monsters can only target Link Monsters for attacks
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    effect::set_value_fn(d, e2, effect_card_value(e2_value));
    card::register_effect(d, c, e2, false);
}
/// `function(c) return c:IsSetCard(SET_CRUSADIA) and c:IsLinkMonster() end`
fn crusadia_link(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_CRUSADIA as u16]) && card::is_link_monster(d, c)
}
/// `e2` condition
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&crusadia_link), p, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `function(e,c) return not c:IsLinkMonster() end`
fn e2_value(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    (!card::is_link_monster(d, c)) as i64
}
/// `s.spcostfilter(c,e,tp)`
fn spcostfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    if !card::is_set_card(d, c, &[SET_CRUSADIA as u16, SET_WORLD_LEGACY as u16]) {
        return false;
    }
    if duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 <= 0 {
        return false;
    }
    let code = card::get_original_code_rule(d, c).first().copied().unwrap_or(0) as i64;
    duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, code)), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.spfilter(c,e,tp,code)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, code: i64) -> bool {
    card::is_set_card(d, c, &[SET_CRUSADIA as u16, SET_WORLD_LEGACY as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && !card::is_original_code_rule(d, c, &[code as u32])
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chk == Some(0) {
        return true;
    }
    if duel::check_release_group_cost(d, tp, Some(&|d, c| spcostfilter(d, c, e, tp)), 1, 1, false, None, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
    {
        let g = duel::select_release_group_cost(d, tp, Some(&|d, c| spcostfilter(d, c, e, tp)), 1, 1, false, None, NO_EX);
        let sc = group::get_first(d, &g).nil();
        let code = card::get_original_code_rule(d, sc).first().copied().unwrap_or(0) as i64;
        effect::set_label(d, e, &[code]);
        duel::release(d, sc, REASON_COST, None);
    } else {
        effect::set_label(d, e, &[-1]);
    }
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chk == Some(0) {
        return true;
    }
    if effect::get_label(d, e) > 0 {
        effect::set_category(d, e, CATEGORY_SPECIAL_SUMMON);
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    } else {
        effect::set_category(d, e, 0);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let label = effect::get_label(d, e);
    if label == -1 || duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let f = aux::necro_valley_filter(move |d, c| spfilter(d, c, e, tp, label));
    let g = duel::select_matching_card(d, tp, Some(&f), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
