//! Machine Lord Ür
use super::prelude::*;

pub const ID: u32 = 96938777;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // attack all
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_ATTACK_ALL)
    effect::set_code(d, e1, EFFECT_ATTACK_ALL);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // attack all
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_NO_BATTLE_DAMAGE)
    effect::set_code(d, e2, EFFECT_NO_BATTLE_DAMAGE);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
