//! Cash Back
use super::prelude::*;

pub const ID: u32 = 59957503;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_TODECK);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    aux::global_check(d, ID, |d| {
        set_s_var(d, ID, 0, Value::Nil);
        let ge1 = effect::create_effect(d, c);
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, ge1, EVENT_PAY_LPCOST);
        effect::set_operation(d, ge1, Some(operation(checkop)));
        duel::register_effect(d, ge1, 0);
    });
}
fn checkop(d: &mut Duel, _a: &EffArgs) {
    let cid = duel::get_current_chain(d, false);
    if cid > 0 {
        let info = duel::get_chain_info(d, cid as u8, &[CHAININFO_CHAIN_ID]);
        set_s_var(d, ID, 0, info[0].clone());
    }
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let re = a.re.nil();
    let info = duel::get_chain_info(d, 0, &[CHAININFO_CHAIN_ID]);
    a.rp != tp
        && lua_eq(&info[0], &s_var(d, ID, 0))
        && (effect::is_monster_effect(d, re) || effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE))
        && duel::is_chain_negatable(d, a.ev as u8)
}
/// Value equality as Lua's `==` (nil equals only nil; numbers compare by value).
fn lua_eq(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Nil, Value::Nil) => true,
        (Value::Int(x), Value::Int(y)) => x == y,
        _ => false,
    }
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    let re = a.re.nil();
    let rh = effect::get_handler(d, re).nil();
    if card::is_relate_to_effect(d, rh, re) {
        duel::set_operation_info(d, 0, CATEGORY_TODECK, a.eg.clone(), 1, 0, 0);
    }
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let re = a.re.nil();
    let rh = effect::get_handler(d, re);
    if duel::negate_activation(d, a.ev as u8) && card::is_relate_to_effect(d, rh.nil(), re) {
        let ec = rh.nil();
        card::cancel_to_grave(d, ec, true);
        duel::send_to_deck(d, ec, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
