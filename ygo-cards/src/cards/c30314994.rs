//! Element Dragon
use super::prelude::*;

pub const ID: u32 = 30314994;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atk
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_value(d, e1, 500);
    effect::set_condition(d, e1, Some(condition(atkcon)));
    card::register_effect(d, c, e1, false);
    // chain attack
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYING);
    effect::set_condition(d, e2, Some(condition(atcon)));
    effect::set_operation(d, e2, Some(operation(atop)));
    card::register_effect(d, c, e2, false);
}
/// `s.atkcon(e)`
fn atkcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsAttribute,ATTRIBUTE_FIRE),0,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    let f = aux::faceup_filter(|d, c| card::is_attribute(d, c, ATTRIBUTE_FIRE));
    duel::is_existing_matching_card(d, Some(&f), 0, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
}
/// `s.atcon(e,tp,eg,ep,ev,re,r,rp)`
fn atcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return aux.bdocon(e,tp,eg,ep,ev,re,r,rp) and e:GetHandler():CanChainAttack()
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsAttribute,ATTRIBUTE_WIND),tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
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
    if !card::can_chain_attack(d, h, None, false) {
        return false;
    }
    let f = aux::faceup_filter(|d, c| card::is_attribute(d, c, ATTRIBUTE_WIND));
    duel::is_existing_matching_card(d, Some(&f), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
}
/// `s.atop(e,tp,eg,ep,ev,re,r,rp)`
fn atop(d: &mut Duel, _a: &EffArgs) {
    // Duel.ChainAttack()
    duel::chain_attack(d, None);
}
