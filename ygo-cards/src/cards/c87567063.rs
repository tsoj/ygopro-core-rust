//! Rebellion
use super::prelude::*;

pub const ID: u32 = 87567063;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_CONTROL);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    card::register_effect(d, c, e1, false);
}
fn condition_fn(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_battle_phase(d, None)
}
fn ctrl_filter(d: &mut Duel, c: CardId) -> bool {
    card::is_controler_can_be_changed(d, c, false, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && card::is_controler_can_be_changed(d, chkc, false, None);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&ctrl_filter), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONTROL as u64);
    let g = duel::select_target(d, tp, Some(&ctrl_filter), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(g), 1, 0, 0);
    true
}
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::get_control(d, CardSet::from_card(tc), tp, PHASE_BATTLE as u16, 1, None, None);
    }
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    effect::set_target(d, e1, Some(effect_card_fn(ftarget)));
    let fid = card::get_field_id(d, tc);
    effect::set_label(d, e1, &[fid as i64]);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, tp);
}
/// `function(e,c) return e:GetLabel()~=c:GetFieldID() end`
fn ftarget(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    effect::get_label(d, e) != card::get_field_id(d, c) as i64
}
