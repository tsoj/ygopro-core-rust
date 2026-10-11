//! Element Magician
use super::prelude::*;

pub const ID: u32 = 65260293;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // control
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_CANNOT_CHANGE_CONTROL)
    effect::set_code(d, e1, EFFECT_CANNOT_CHANGE_CONTROL);
    // e1:SetCondition(s.ctlcon)
    effect::set_condition(d, e1, Some(condition(ctlcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // chain attack
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYING);
    // e2:SetCondition(s.atcon)
    effect::set_condition(d, e2, Some(condition(atcon)));
    // e2:SetOperation(s.atop)
    effect::set_operation(d, e2, Some(operation(atop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.filter(c,att)
fn filter(d: &mut Duel, c: CardId, att: u32) -> bool {
    // return c:IsFaceup() and c:IsAttribute(att)
    card::is_faceup(d, c) && card::is_attribute(d, c, att)
}
// function s.ctlcon(e)
fn ctlcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.filter,0,LOCATION_MZONE,LOCATION_MZONE,1,nil,ATTRIBUTE_WATER)
    duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, ATTRIBUTE_WATER)), 0, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
}
// function s.atcon(e,tp,eg,ep,ev,re,r,rp)
fn atcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return aux.bdocon(e,tp,eg,ep,ev,re,r,rp) and e:GetHandler():CanChainAttack()
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil,ATTRIBUTE_WIND)
    let args = [Value::Effect(a.e), Value::Int(tp as i64), Value::Nil, Value::Nil, Value::Nil, Value::Nil, Value::Nil, Value::Nil];
    if !aux::bdocon().call(d, &args).truthy() {
        return false;
    }
    let c = effect::get_handler(d, a.e).nil();
    card::can_chain_attack(d, c, None, false)
        && duel::is_existing_matching_card(d, Some(&|d, x| filter(d, x, ATTRIBUTE_WIND)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
}
// function s.atop(e,tp,eg,ep,ev,re,r,rp)
fn atop(d: &mut Duel, _a: &EffArgs) {
    // Duel.ChainAttack()
    duel::chain_attack(d, None);
}
