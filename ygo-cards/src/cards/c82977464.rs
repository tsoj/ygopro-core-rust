//! S-Force Specimen
use super::prelude::*;

pub const ID: u32 = 82977464;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_S_FORCE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon 1 "Security Force" monster, that is banished or in your GY
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    card::register_effect(d, c, e1, false);
    // Move 1 "Security Force" to another of your MMZ
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(seqtg)));
    effect::set_operation(d, e2, Some(operation(seqop)));
    effect::set_hint_timing(d, e2, 0, Some(TIMING_END_PHASE));
    card::register_effect(d, c, e2, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_field_group_count(d, a.tp, 0, LOCATION_MZONE) > 0
}
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, zone: u32) -> bool {
    card::is_set_card(d, c, &[SET_S_FORCE as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(tp), Some(zone))
        && card::is_faceup(d, c)
}
fn getzones(d: &mut Duel, tp: u8) -> u32 {
    let lg = duel::get_matching_group(d, None, tp, 0, LOCATION_MZONE, NO_EX);
    let mut zone = 0;
    for tc in group::iter(d, &lg) {
        zone |= card::get_column_zone(d, tc, LOCATION_MZONE, 0, 0, Some(tp));
    }
    zone & ZONES_MMZ
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    if let Some(chkc) = a.chkc() {
        let zone = getzones(d, tp);
        return card::is_location(d, chkc, LOCATION_GRAVE | LOCATION_REMOVED) && card::is_controler(d, chkc, tp) && filter(d, chkc, e, tp, zone);
    }
    if a.chk == Some(0) {
        let zone = getzones(d, tp);
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, e, tp, zone)), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let zone = getzones(d, tp);
    let g = duel::select_target(d, tp, Some(&|d, c| filter(d, c, e, tp, zone)), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        let zone = getzones(d, tp);
        duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, Some(zone));
    }
}
fn seqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_S_FORCE as u16]))), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 1));
    duel::select_target(d, tp, Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_S_FORCE as u16]))), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
fn seqop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    let tc = duel::first_target(d).nil();
    if card::is_immune_to_effect(d, tc, e)
        || !card::is_relate_to_effect(d, tc, e)
        || card::is_controler(d, tc, 1 - tp)
        || duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 == 0
    {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOZONE as u64);
    let zone = duel::select_disable_field(d, tp, 1, LOCATION_MZONE, 0, Some(0), false);
    if zone == 0 {
        // math.log(nil, 2) is an error in Lua
        crate::lua_error("bad argument #1 to 'log' (number expected, got nil)");
    }
    // math.log(zone, 2) of a single zone bit
    duel::move_sequence(d, tc, zone.trailing_zeros() as u8, None);
}
