//! Fire Formation - Tensu
use super::prelude::*;

pub const ID: u32 = 10719350;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // extra summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(LOCATION_HAND|LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_HAND | LOCATION_MZONE, 0);
    // e2:SetCode(EFFECT_EXTRA_SUMMON_COUNT)
    effect::set_code(d, e2, EFFECT_EXTRA_SUMMON_COUNT);
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsRace,RACE_BEASTWARRIOR))
    effect::set_target(d, e2, Some(effect_card_fn(beastwarrior)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e3, LOCATION_MZONE, 0);
    // e3:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    // e3:SetTarget(aux.TargetBoolFunction(Card.IsRace,RACE_BEASTWARRIOR))
    effect::set_target(d, e3, Some(effect_card_fn(beastwarrior)));
    // e3:SetValue(100)
    effect::set_value(d, e3, 100);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// aux.TargetBoolFunction(Card.IsRace,RACE_BEASTWARRIOR)
fn beastwarrior(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_race(d, c, RACE_BEASTWARRIOR)
}
