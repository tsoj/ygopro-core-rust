//! Ljosalf of the Nordic Alfar
use super::prelude::*;

pub const ID: u32 = 40666140;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NORDIC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // spsummon
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    let lv = card::get_level(d, c);
    card::is_faceup(d, c) && lv > 0
        && duel::is_existing_matching_card(d, Some(&|d, cc| filter2(d, cc, lv, e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX)
}
fn filter2(d: &mut Duel, c: CardId, lv: u32, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_NORDIC as u16]) && card::get_level(d, c) <= lv
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && filter(d, chkc, e, tp);
    }
    if a.chk == Some(0) {
        let h = effect::get_handler(d, e).nil();
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_MZONE, 0, 1, h.into());
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    let h = effect::get_handler(d, e).nil();
    duel::select_target(d, tp, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, h.into());
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    let tc = duel::first_target(d).nil();
    if !card::is_relate_to_effect(d, tc, a.e) || card::is_facedown(d, tc) {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let lv = card::get_level(d, tc);
    let e = a.e;
    let sg = duel::select_matching_card(d, tp, Some(&|d, c| filter2(d, c, lv, e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    if !sg.is_empty() {
        duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
