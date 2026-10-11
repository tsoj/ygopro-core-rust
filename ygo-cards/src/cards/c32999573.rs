//! Xyz Override
use super::prelude::*;

pub const ID: u32 = 32999573;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Remove detach cost
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    effect::set_property(d, e2, EFFECT_FLAG_BOTH_SIDE, 0);
    effect::set_code(d, e2, EFFECT_OVERLAY_REMOVE_REPLACE);
    effect::set_range(d, e2, LOCATION_FZONE);
    effect::set_condition(d, e2, Some(condition(rcon)));
    effect::set_operation(d, e2, Some(condition(rop)));
    card::register_effect(d, c, e2, false);
}
fn rcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    if card::get_flag_effect(d, h, ID + a.ep as u32) != 0 {
        return false;
    }
    if a.r & REASON_COST == 0 {
        return false;
    }
    let re = a.re.nil();
    if !effect::is_activated(d, re) {
        return false;
    }
    if !effect::is_active_type(d, re, TYPE_XYZ) {
        return false;
    }
    let rh = effect::get_handler(d, re).nil();
    if (card::get_overlay_count(d, rh) as i64) < a.ev as i64 - 1 {
        return false;
    }
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_remove(d, c, Some(a.tp), Some(POS_FACEDOWN as u8), None)), a.tp, LOCATION_HAND, 0, 1, NO_EX)
}
fn rop(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_able_to_remove(d, c, Some(tp), Some(POS_FACEDOWN as u8), None)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID + a.ep as u32, RESET_PHASE | PHASE_END, 0, 1, 0, 0);
    duel::remove(d, g, POS_FACEDOWN as u8, REASON_COST, None, None) > 0
}
