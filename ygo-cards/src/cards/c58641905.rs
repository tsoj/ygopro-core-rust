//! Ring of Defense
use super::prelude::*;

pub const ID: u32 = 58641905;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // reflect
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    if !effect::is_trap_effect(d, re) {
        return false;
    }
    // aux.damcon1(e,tp,eg,ep,ev,re,r,rp)
    let args = [
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        Value::Effect(re),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
    ];
    aux::damcon1().call(d, &args).truthy()
}
fn op(d: &mut Duel, a: &EffArgs) {
    let cid = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_CHAIN_ID]).first().map_or(0, |v| v.int());
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CHANGE_DAMAGE);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e1, 1, 1);
    effect::set_label(d, e1, &[cid]);
    effect::set_value_fn(d, e1, lua_fn(refcon));
    effect::set_reset(d, e1, RESET_CHAIN, 1);
    duel::register_effect(d, e1, a.tp);
}
fn refcon(d: &mut Duel, a: Args) -> i64 {
    let e = a.e(0);
    let val = a.int(2);
    let r = a.int(3) as u32;
    let cc = duel::get_current_chain(d, false);
    if cc == 0 || (r & REASON_EFFECT) == 0 {
        return 0;
    }
    let cid = duel::get_chain_info(d, 0, &[CHAININFO_CHAIN_ID]).first().map_or(0, |v| v.int());
    if cid == effect::get_label(d, e) {
        return 0;
    }
    val
}
