//! Crop Circles
use super::prelude::*;

pub const ID: u32 = 24082387;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ALIEN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "Alien" monster from your Deck whose Level is equal to the total original Levels of the monsters sent to the GY to activate this card
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON | CATEGORY_DAMAGE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    card::register_effect(d, c, e1, false);
}
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    card::has_level(d, c) && card::is_able_to_grave_as_cost(d, c) && card::is_monster_card(d, c)
}
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    let res = duel::get_m_zone_count(d, tp, sg.into(), None, None, None).0 > 0 && {
        let lv = group::get_sum(d, sg, &|d, c| card::get_original_level(d, c) as i64);
        duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e.nil(), tp, lv)), tp, LOCATION_DECK, 0, 1, NO_EX)
    };
    (res, false)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: i64) -> bool {
    card::is_set_card(d, c, &[SET_ALIEN as u16]) && card::is_level(d, c, &[lv as u32]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    effect::set_label(d, e, &[-100]);
    let cg = duel::get_matching_group(d, Some(&costfilter), tp, LOCATION_MZONE, 0, NO_EX);
    let n = cg.len();
    if a.chk == Some(0) {
        return n > 0 && matches!(aux::select_unselect_group(d, &cg, Some(e), tp, Some(1), Some(n), Some(&rescon), 0, tp, None, None, None, false), Err(true));
    }
    let g = aux::select_unselect_group(d, &cg, Some(e), tp, Some(1), Some(n), Some(&rescon), 1, tp, Some(HINTMSG_TOGRAVE as u64), Some(&rescon), None, false).unwrap_or_default();
    duel::send_to_grave(d, g.clone(), REASON_COST, None, None);
    let lv = group::get_sum(d, &g, &|d, c| card::get_original_level(d, c) as i64);
    effect::set_label(d, e, &[lv]);
    true
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chk == Some(0) {
        let res = effect::get_label(d, e) == -100;
        effect::set_label(d, e, &[0]);
        return res;
    }
    duel::set_target_param(d, effect::get_label(d, e) as u32);
    effect::set_label(d, e, &[0]);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, tp, 2000);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let mut sp_chk = 0;
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let lv = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
        let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, lv)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        if !g.is_empty() {
            sp_chk = duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
    if sp_chk == 0 {
        duel::damage(d, tp, 2000, REASON_EFFECT, false, None);
    }
}
