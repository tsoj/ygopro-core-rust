//! Jester Lord
use super::prelude::*;

pub const ID: u32 = 72992744;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.atkcon)
    effect::set_condition(d, e1, Some(condition(atkcon)));
    // e1:SetValue(s.atkval)
    effect::set_value_fn(d, e1, effect_value(atkval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.atkcon(e)
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsExistingMatchingCard(nil,0,LOCATION_MZONE,LOCATION_MZONE,1,e:GetHandler())
    let c = effect::get_handler(d, a.e).nil();
    !duel::is_existing_matching_card(d, None, 0, LOCATION_MZONE, LOCATION_MZONE, 1, c.into())
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetSequence()<5
    card::get_sequence(d, c) < 5
}
// function s.atkval(e,c)
fn atkval(d: &mut Duel, _e: EffectId) -> i64 {
    // return Duel.GetMatchingGroupCount(s.filter,0,LOCATION_SZONE,LOCATION_SZONE,nil)*1000
    duel::get_matching_group_count(d, Some(&filter), 0, LOCATION_SZONE, LOCATION_SZONE, NO_EX) as i64 * 1000
}
