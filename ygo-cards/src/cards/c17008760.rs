//! Materiactor Annulus
use super::prelude::*;

pub const ID: u32 = 17008760;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Detach 1 Xyz material from your monster
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON | CATEGORY_SET);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::check_remove_overlay_card(d, tp, 1, 0, 1, REASON_EFFECT, None);
    }
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_GRAVE as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
// Detach 1 Xyz material from your monster
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    if duel::remove_overlay_card(d, tp, 1, 0, 1, 1, REASON_EFFECT, None) < 1 {
        return;
    }
    // Set the detached card
    let operated = duel::get_operated_group(d);
    let rc = group::get_first(d, &operated).nil();
    if !(card::is_location(d, rc, LOCATION_GRAVE) && card::is_controler(d, rc, tp)) {
        return;
    }
    if card::is_monster(d, rc)
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && card::is_can_be_special_summoned(d, rc, e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), None, None)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
    {
        duel::break_effect(d);
        duel::special_summon(d, rc, 0, tp, tp, false, false, POS_FACEDOWN_DEFENSE as u8, None);
        duel::confirm_cards(d, 1 - tp, rc);
    } else if card::is_s_setable(d, rc, false, None) && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
        duel::break_effect(d);
        duel::s_set(d, tp, rc, None, true);
    }
}
