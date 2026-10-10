//! Dedication through Light and Darkness
use super::prelude::*;

pub const ID: u32 = 69542930;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_DARK_MAGICIAN, 40737112], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cfilter(d: &mut Duel, c: CardId, ft: i32, tp: u8) -> bool {
    card::is_code(d, c, &[CARD_DARK_MAGICIAN])
        && (ft > 0 || (card::get_sequence(d, c) < 5 && card::is_controler(d, c, tp)))
        && (card::is_faceup(d, c) || card::is_controler(d, c, tp))
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    effect::set_label(d, e, &[1]);
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, Some(&|d, x| cfilter(d, x, ft, tp)), 1, 1, false, None, NO_EX);
    }
    let g = duel::select_release_group_cost(d, tp, Some(&|d, x| cfilter(d, x, ft, tp)), 1, 1, false, None, NO_EX);
    duel::release(d, g, REASON_COST, None);
    true
}
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_code(d, c, &[40737112]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        if effect::get_label(d, a.e) == 0 && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return false;
        }
        effect::set_label(d, a.e, &[0]);
        let (e, tp) = (a.e, a.tp);
        return duel::is_existing_matching_card(d, Some(&|d, x| filter(d, x, e, tp)), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(|d, x| filter(d, x, e, tp))), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
