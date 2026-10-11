//! Dirge of the Lost Dragon
use super::prelude::*;

pub const ID: u32 = 40971261;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // halving
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_CHAINING);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_operation(d, e2, Some(condition(regop)));
    effect::set_label_object(d, e2, Value::Effect(e1));
    card::register_effect(d, c, e2, false);
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e3, EVENT_CHAIN_SOLVED);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_condition(d, e3, Some(condition(damcon)));
    effect::set_operation(d, e3, Some(operation(damop)));
    effect::set_label_object(d, e3, Value::Effect(e1));
    card::register_effect(d, c, e3, false);
    // protection
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e4, LOCATION_SZONE);
    effect::set_code(d, e4, EFFECT_INDESTRUCTABLE_EFFECT);
    effect::set_condition(d, e4, Some(condition(indescond)));
    effect::set_value(d, e4, 1);
    card::register_effect(d, c, e4, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CODE as u64);
    let ac = duel::announce_card(d, tp, &[]);
    duel::set_target_param(d, ac as u32);
    duel::set_operation_info(d, 0, CATEGORY_ANNOUNCE, None, 0, tp, ANNOUNCE_CARD as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let ac = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
    effect::set_label(d, a.e, &[ac]);
    let c = effect::get_handler(d, a.e).nil();
    card::set_hint(d, c, CHINT_CARD as u8, ac as u64);
    duel::adjust_instantly(d, Some(c));
}
/// `function(e,tp,eg,ep,ev,re,r,rp) ... return true end` (the operation returns a value)
fn regop(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    let lab = effect::get_label(d, lo);
    let code = card::get_code(d, rc).first().copied();
    if code.map(|x| x as i64) == Some(lab) {
        let c = effect::get_handler(d, a.e).nil();
        card::register_flag_effect(d, c, ID, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET) | RESET_CHAIN, 0, 1, 0, 0);
        return true;
    }
    false
}
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if card::get_flag_effect(d, c, ID) == 0 {
        return false;
    }
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    let lab = effect::get_label(d, lo);
    let code = card::get_code(d, rc).first().copied();
    code.map(|x| x as i64) == Some(lab)
}
fn damop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let rp = a.rp;
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    let lp = duel::get_lp(d, rp);
    duel::set_lp(d, rp, lua_int(lp as f64 / 2.0) as i32);
    card::register_flag_effect(d, c, ID + 1, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    effect::set_range(d, e1, LOCATION_SZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(tgcon)));
    effect::set_operation(d, e1, Some(operation(tgop)));
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, c, e1, false);
}
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, c, ID + 1) != 0
}
fn tgop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    duel::send_to_grave(d, c, REASON_EFFECT, None, None);
}
fn indescond(d: &mut Duel, a: &EffArgs) -> bool {
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, None, p, 0, LOCATION_MZONE, 1, NO_EX)
}
