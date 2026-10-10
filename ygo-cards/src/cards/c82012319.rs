//! Scrap Golem
use super::prelude::*;

pub const ID: u32 = 82012319;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SCRAP], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 Level 4 or lower "Scrap" monster
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_level_below(d, c, 4)
        && card::is_set_card(d, c, &[SET_SCRAP as u16])
        && (card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
            || card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(1 - tp), None))
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        if !(card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp)) {
            return false;
        }
        // `s.filter` is not defined in this script: the Lua call is nil
        crate::lua_error("attempt to call a nil value");
    }
    if a.chk == Some(0) {
        return (duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            || duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0)
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    let b1 = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && card::is_can_be_special_summoned(d, tc, a.e, 0, a.tp, false, false, None, None, None);
    let b2 = duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && card::is_can_be_special_summoned(d, tc, a.e, 0, a.tp, false, false, Some(POS_FACEUP as u8), Some(1 - a.tp), None);
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, 0);
    let op = if b1 && b2 {
        duel::select_option(d, a.tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2)])
    } else if b1 {
        duel::select_option(d, a.tp, true, &[aux::stringid(ID, 1)])
    } else if b2 {
        duel::select_option(d, a.tp, true, &[aux::stringid(ID, 2)]) + 1
    } else {
        return;
    };
    if op == 0 {
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    } else {
        duel::special_summon(d, tc, 0, a.tp, 1 - a.tp, false, false, POS_FACEUP as u8, None);
    }
}
