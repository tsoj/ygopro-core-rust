//! Mind Trust
use super::prelude::*;

pub const ID: u32 = 38680149;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_TOHAND);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_label(d, e1, &[0]);
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    effect::set_label(d, a.e, &[100]);
    true
}
fn filter1(d: &mut Duel, c: CardId, _e: EffectId, tp: u8) -> bool {
    let lv = card::get_level(d, c).div_euclid(2);
    lv > 0 && duel::is_existing_target(d, Some(&|d, x| filter2(d, x, lv as u32)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
fn filter2(d: &mut Duel, c: CardId, lv: u32) -> bool {
    card::is_level_below(d, c, lv) && card::is_type(d, c, TYPE_TUNER) && card::is_able_to_hand(d, c, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if let Some(chkc) = a.chkc() {
        let lbl = effect::get_label(d, e) as u32;
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && filter2(d, chkc, lbl);
    }
    if a.chk == Some(0) {
        if effect::get_label(d, e) != 100 {
            return false;
        }
        effect::set_label(d, e, &[0]);
        return duel::check_release_group_cost(
            d,
            tp,
            Some(&|d, c| filter1(d, c, e, tp)),
            1,
            1,
            false,
            None,
            NO_EX,
        );
    }
    let rg = duel::select_release_group_cost(d, tp, Some(&|d, c| filter1(d, c, e, tp)), 1, 1, false, None, NO_EX);
    let first = group::get_first(d, &rg).nil();
    let lv = card::get_level(d, first).div_euclid(2);
    effect::set_label(d, e, &[lv as i64]);
    duel::release(d, rg, REASON_COST, None);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_target(d, tp, Some(&|d, x| filter2(d, x, lv as u32)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - a.tp, tc);
    }
}
