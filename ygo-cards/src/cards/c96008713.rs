//! Magical Arm Shield
use super::prelude::*;

pub const ID: u32 = 96008713;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_CONTROL);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp) && duel::get_field_group_count(d, a.tp, LOCATION_MZONE, 0) > 0
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_controler_can_be_changed(d, c, false, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let atr = duel::get_attacker(d);
    let ex = atr.map_or(NO_EX, |c| c.into());
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, 0, LOCATION_MZONE, 1, ex);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONTROL as u64);
    let g = duel::select_target(d, tp, Some(&filter), tp, 0, LOCATION_MZONE, 1, 1, false, ex);
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, g, 1, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let tc = duel::first_target(d);
    let at = duel::get_attacker(d);
    if card::is_relate_to_effect(d, tc.nil(), a.e) && card::is_faceup(d, tc.nil()) && duel::get_control(d, tc.nil(), tp, PHASE_BATTLE as u16, 1, None, None) != 0 {
        if card::can_attack(d, at.nil()) && !card::is_immune_to_effect(d, at.nil(), a.e) {
            duel::calculate_damage(d, at.nil(), Some(tc.nil()), false);
        }
    }
}
