//! Aegis of Gaia
use super::prelude::*;

pub const ID: u32 = 47060347;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_category(d, e1, CATEGORY_RECOVER);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(rectg)));
    effect::set_operation(d, e1, Some(operation(recop)));
    card::register_effect(d, c, e1, false);
    // Damage when leaving
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e2, EVENT_LEAVE_FIELD_P);
    effect::set_operation(d, e2, Some(operation(checkop)));
    card::register_effect(d, c, e2, false);
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e3, EVENT_LEAVE_FIELD);
    effect::set_label_object(d, e3, Value::Effect(e2));
    effect::set_operation(d, e3, Some(operation(leave)));
    card::register_effect(d, c, e3, false);
}
fn rectg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_target_player(d, a.tp);
    duel::set_target_param(d, 3000);
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, 3000);
    true
}
fn recop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::recover(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
fn checkop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_disabled(d, c) || !card::is_status(d, c, STATUS_EFFECT_ENABLED) {
        effect::set_label(d, a.e, &[1]);
    } else {
        effect::set_label(d, a.e, &[0]);
    }
}
fn leave(d: &mut Duel, a: &EffArgs) {
    let e2 = effect::get_label_object(d, a.e).as_effect().nil();
    if effect::get_label(d, e2) == 0 {
        let c = effect::get_handler(d, a.e).nil();
        let p = card::get_previous_controler(d, c);
        duel::damage(d, p, 3000, REASON_EFFECT, false, None);
    }
}
