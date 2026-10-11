//! Kishido Spirit
use super::prelude::*;

pub const ID: u32 = 60519422;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // indes
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetTarget(s.indtg)
    effect::set_target(d, e2, Some(effect_card_fn(indtg)));
    // e2:SetValue(s.indval)
    effect::set_value_fn(d, e2, effect_card_value(indval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn indtg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // e:SetLabel(c:GetAttack())
    let atk = card::get_attack(d, c);
    effect::set_label(d, e, &[atk as i64]);
    // return true
    true
}
fn indval(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    // return c:GetAttack()==e:GetLabel()
    let atk = card::get_attack(d, c) as i64;
    (atk == effect::get_label(d, e)) as i64
}
