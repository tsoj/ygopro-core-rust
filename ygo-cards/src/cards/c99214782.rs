//! Steelswarm Sentinel
use super::prelude::*;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot trigger
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_TRIGGER)
    effect::set_code(d, e1, EFFECT_CANNOT_TRIGGER);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(effect_card_fn(target)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsAttackPos()
    let h = effect::get_handler(d, a.e).nil();
    card::is_attack_pos(d, h)
}
/// `s.target(e,c)`
fn target(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:GetLevel()>=5 and c:IsSpecialSummoned()
    card::get_level(d, c) >= 5 && card::is_special_summoned(d, c)
}
