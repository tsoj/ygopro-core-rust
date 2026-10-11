//! Parasitic Ticky
//! 寄生体ダニー
use super::prelude::*;

pub const ID: u32 = 87978805;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atk/def
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_SET_ATTACK)
    effect::set_code(d, e1, EFFECT_SET_ATTACK);
    // e1:SetValue(s.val)
    effect::set_value_fn(d, e1, effect_card_value(val));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_SET_DEFENSE)
    effect::set_code(d, e2, EFFECT_SET_DEFENSE);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.val(e,c)
fn val(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return Duel.GetMatchingGroupCount(Card.IsType,c:GetControler(),LOCATION_MZONE,LOCATION_MZONE,nil,TYPE_TOKEN)*500
    let tp = card::get_controler(d, c);
    duel::get_matching_group_count(d, Some(&|d, c| card::is_type(d, c, TYPE_TOKEN)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX) as i64 * 500
}
