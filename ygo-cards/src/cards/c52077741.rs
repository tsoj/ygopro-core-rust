//! Obnoxious Celtic Guard
use super::prelude::*;

pub const ID: u32 = 52077741;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // indes
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetValue(s.indes)
    effect::set_value_fn(d, e1, effect_card_value(indes));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.indes(e,c)`
fn indes(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return c:GetAttack()>=1900
    (card::get_attack(d, c) >= 1900) as i64
}
