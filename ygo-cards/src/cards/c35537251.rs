//! Battlin' Boxer Shadow
use super::prelude::*;

pub const ID: u32 = 35537251;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BATTLIN_BOXER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Detach material and Special Summon
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_BATTLIN_BOXER as u16]) && card::is_type(d, c, TYPE_XYZ)
        && card::get_overlay_count(d, c) > 0
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, h, a.e, 0, a.tp, false, false, None, None, None);
    }
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DEATTACHFROM as u64);
    let xyzg = duel::select_matching_card(d, a.tp, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let Some(tc) = group::get_first(d, &xyzg) else {
        return;
    };
    duel::hint_selection(d, &xyzg, true);
    let mg = card::get_overlay_group(d, tc);
    let ct = mg.len();
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVEXYZ as u64);
    let sg = group::select(d, &mg, a.tp, 1, 1, false, NO_EX).nil();
    if !sg.is_empty() && duel::send_to_grave(d, &sg, REASON_EFFECT, None, None) > 0 && card::get_overlay_count(d, tc) < ct {
        let c = effect::get_handler(d, a.e).nil();
        if !card::is_relate_to_effect(d, c, a.e) {
            return;
        }
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
