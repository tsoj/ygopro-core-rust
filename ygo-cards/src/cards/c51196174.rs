//! The Calculator
use super::prelude::*;

pub const ID: u32 = 51196174;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SET_ATTACK)
    effect::set_code(d, e1, EFFECT_SET_ATTACK);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(s.atkval)
    effect::set_value_fn(d, e1, effect_card_value(atkval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,c:GetControler(),LOCATION_MZONE,0,nil)
    let p = card::get_controler(d, c);
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x)), p, LOCATION_MZONE, 0, NO_EX);
    // return g:GetSum(Card.GetLevel)*300
    group::get_sum(d, &g, &|d, x| card::get_level(d, x) as i64) * 300
}
