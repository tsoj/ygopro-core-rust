//! Chronomaly City Babylon
use super::prelude::*;

pub const ID: u32 = 4357063;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CHRONOMALY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "Chronomaly" monster from the GY
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e2, LOCATION_FZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
}
fn costfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_CHRONOMALY as u16])
        && card::is_able_to_remove_as_cost(d, c, None)
        && card::is_monster(d, c)
        && card::has_level(d, c)
        && aux::sp_elim_filter(d, c, true, false)
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
        && {
            let lv = card::get_level(d, c) as i64;
            duel::is_existing_target(d, Some(&|d, x| spfilter(d, x, e, tp, lv)), tp, LOCATION_GRAVE, 0, 1, c.into())
        }
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: i64) -> bool {
    card::is_set_card(d, c, &[SET_CHRONOMALY as u16])
        && card::is_monster(d, c)
        && card::is_level(d, c, &[lv as u32])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE)
            && card::is_controler(d, chkc, tp)
            && spfilter(d, chkc, a.e, tp, effect::get_label(d, a.e));
    }
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| costfilter(d, c, a.e, tp)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| costfilter(d, c, a.e, tp)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    duel::remove(d, &g, POS_FACEUP as u8, REASON_COST, None, None);
    let first = group::get_first(d, &g).nil();
    let lv = card::get_level(d, first) as i64;
    effect::set_label(d, a.e, &[lv]);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp, lv)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
