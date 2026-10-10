//! Different Dimension Dragon
use super::prelude::*;

pub const ID: u32 = 50939127;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // indestructable
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    // e1:SetValue(s.ind1)
    effect::set_value_fn(d, e1, lua_fn(ind1));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    // e2:SetValue(s.ind2)
    effect::set_value_fn(d, e2, effect_card_fn(ind2));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.ind1(e,re,rp,c)`
fn ind1(d: &mut Duel, a: Args) -> bool {
    // return not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) and re:IsSpellTrapEffect()
    let re = a.v(1).as_effect().nil();
    !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) && effect::is_spell_trap_effect(d, re)
}
/// `s.ind2(e,c)`
fn ind2(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsAttackBelow(1900)
    card::is_attack_below(d, c, 1900)
}
