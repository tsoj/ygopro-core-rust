//! Energy-Absorbing Monolith
//! エネルギー吸収板
use super::prelude::*;

pub const ID: u32 = 57006589;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // reflect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`: `return ep~=tp and aux.damcon1(e,tp,eg,ep,ev,re,r,rp)`
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    if a.ep == a.tp {
        return false;
    }
    let args = [
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map(Value::Group).unwrap_or(Value::Nil),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        a.re.map(Value::Effect).unwrap_or(Value::Nil),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
    ];
    aux::damcon1().call(d, &args).truthy()
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local cid=Duel.GetChainInfo(ev,CHAININFO_CHAIN_ID)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_CHAIN_ID]);
    let cid = info[0].int();
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_REVERSE_DAMAGE)
    effect::set_code(d, e1, EFFECT_REVERSE_DAMAGE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetLabel(cid)
    effect::set_label(d, e1, &[cid]);
    // e1:SetValue(s.refcon)
    effect::set_value_fn(d, e1, lua_fn(refcon));
    // e1:SetReset(RESET_CHAIN)
    effect::set_reset(d, e1, RESET_CHAIN, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `s.refcon(e,re,r,rp,rc)`
fn refcon(d: &mut Duel, a: Args) -> Value {
    // local cc=Duel.GetCurrentChain()
    let cc = duel::get_current_chain(d, false);
    let r = a.int(2) as u32;
    // if cc==0 or (r&REASON_EFFECT)==0 then return end
    if cc == 0 || (r & REASON_EFFECT) == 0 {
        return Value::Nil;
    }
    // local cid=Duel.GetChainInfo(0,CHAININFO_CHAIN_ID)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_CHAIN_ID]);
    // return cid==e:GetLabel()
    let label = effect::get_label(d, a.e(0));
    Value::Bool(info[0].int() == label)
}
