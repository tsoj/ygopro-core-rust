//! Ring Announcer
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 19544412;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BATTLIN_BOXER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "Battlin' Boxer" monster from the hand and destory Spells/Traps
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_BATTLIN_BOXER as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, a.tp, LOCATION_ONFIELD as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let sg = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    if !sg.is_empty() && duel::special_summon(d, &sg, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
        let xyz = aux::faceup_filter(|d, c| card::is_type(d, c, TYPE_XYZ));
        let ct = duel::get_matching_group_count(d, Some(&xyz), tp, LOCATION_MZONE, 0, NO_EX);
        let h = effect::get_handler(d, a.e).nil();
        let g = duel::get_matching_group(d, Some(&card::is_spell_trap), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, h.into());
        if ct > 0 && !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
            let dg = group::select(d, &g, tp, 1, ct as u16, false, NO_EX).nil();
            duel::hint_selection(d, &dg, true);
            duel::break_effect(d);
            duel::destroy(d, &dg, REASON_EFFECT, None, None);
        }
    }
}
