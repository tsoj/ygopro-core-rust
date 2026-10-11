//! Theban Nightmare
//! ナイトメアテーベ
use super::prelude::*;

pub const ID: u32 = 51838385;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atk
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetCondition(s.atkcon)
    effect::set_condition(d, e1, Some(condition(atkcon)));
    // e1:SetValue(1500)
    effect::set_value(d, e1, 1500);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetSequence()<5
    card::get_sequence(d, c) < 5
}
/// `s.atkcon(e)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e);
    // return Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)+Duel.GetMatchingGroupCount(s.filter,tp,LOCATION_SZONE,0,nil)==0
    let hand = duel::get_field_group_count(d, tp, LOCATION_HAND, 0);
    let sz = duel::get_matching_group_count(d, Some(&filter), tp, LOCATION_SZONE, 0, NO_EX) as i32;
    hand + sz == 0
}
