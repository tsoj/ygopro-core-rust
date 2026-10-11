//! Rose Witch
use super::prelude::*;

pub const ID: u32 = 23087070;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // double tribute
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_DOUBLE_TRIBUTE)
    effect::set_code(d, e1, EFFECT_DOUBLE_TRIBUTE);
    // e1:SetValue(s.condition)
    effect::set_value_fn(d, e1, effect_card_fn(condition));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.condition(e,c)
fn condition(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsRace(RACE_PLANT)
    card::is_race(d, c, RACE_PLANT as u64)
}
