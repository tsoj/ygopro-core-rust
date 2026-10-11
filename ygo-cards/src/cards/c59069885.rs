//! Embrace of the Tistina
use super::prelude::*;

pub const ID: u32 = 59069885;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TISTINA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e0, false);
    // Change monster to face-down position
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_POSITION + CATEGORY_SET);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_range(d, e1, LOCATION_SZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(poscon)));
    effect::set_target(d, e1, Some(cost_target(postg)));
    effect::set_operation(d, e1, Some(operation(posop)));
    card::register_effect(d, c, e1, false);
    // Take control of 1 face-down position monster
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_CONTROL);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_hint_timing(d, e2, TIMING_END_PHASE, None);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(|d, a| {
        duel::is_phase(d, PHASE_END) && duel::is_existing_matching_card(d, Some(&deffilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
    })));
    effect::set_target(d, e2, Some(cost_target(cttg)));
    effect::set_operation(d, e2, Some(operation(ctop)));
    card::register_effect(d, c, e2, false);
}
fn deffilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_TISTINA as u16]) && card::is_defense_above(d, c, 3000)
}
fn poscon(d: &mut Duel, a: &EffArgs) -> bool {
    if a.rp != 1 - a.tp {
        return false;
    }
    let re = a.re.nil();
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    if effect::get_activate_location(d, re) != LOCATION_MZONE {
        return false;
    }
    let h = effect::get_handler(d, re).nil();
    card::is_relate_to_effect(d, h, re) && duel::is_existing_matching_card(d, Some(&deffilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chkc().is_some() {
        return false;
    }
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    if a.chk == Some(0) {
        return card::is_can_be_effect_target(d, rc, Some(a.e)) && card::is_can_turn_set(d, rc);
    }
    duel::set_target_card(d, rc);
    duel::set_operation_info(d, 0, CATEGORY_POSITION, a.eg.clone(), 1, a.tp, POS_FACEDOWN_DEFENSE as i32);
    true
}
fn posop(d: &mut Duel, _a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, _a.e) && card::is_faceup(d, tc) {
        duel::change_position(d, tc, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
    }
}
fn ctfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_facedown(d, c) && card::is_defense_pos(d, c) && card::is_controler_can_be_changed(d, c, false, None)
}
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && ctfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&ctfilter), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONTROL as u64);
    let g = duel::select_target(d, tp, Some(&ctfilter), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, g, 1, 0, 0);
    true
}
fn ctop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::get_control(d, tc, a.tp, 0, 0, None, None);
    }
}
