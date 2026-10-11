//! Ruin, Queen of Oblivion
use super::prelude::*;

pub const ID: u32 = 46427957;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[8198712], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // chain attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYING);
    // e1:SetCondition(s.atcon)
    effect::set_condition(d, e1, Some(condition(atcon)));
    // e1:SetOperation(s.atop)
    effect::set_operation(d, e1, Some(operation(atop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.atcon(e,tp,eg,ep,ev,re,r,rp)`
fn atcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return aux.bdocon(e,tp,eg,ep,ev,re,r,rp) and e:GetHandler():CanChainAttack()
    let args = [
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        Value::effect(a.re),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
    ];
    if !aux::bdocon().call(d, &args).truthy() {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    card::can_chain_attack(d, h, None, false)
}
/// `s.atop(e,tp,eg,ep,ev,re,r,rp)`
fn atop(d: &mut Duel, _a: &EffArgs) {
    // Duel.ChainAttack()
    duel::chain_attack(d, None);
}
