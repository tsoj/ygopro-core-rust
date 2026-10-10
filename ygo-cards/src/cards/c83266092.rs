//! Grave of the Super Ancient Organism
use super::prelude::*;

pub const ID: u32 = 83266092;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE|TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE | TIMING_END_PHASE));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Level 6 or higher Special Summoned monsters on the field cannot declare attacks
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EFFECT_CANNOT_ATTACK_ANNOUNCE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(effect_card_fn(target)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Players cannot activate the effects of Level 6 or higher Special Summoned monsters
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e3, EFFECT_CANNOT_ACTIVATE);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetTargetRange(1,1)
    effect::set_target_range(d, e3, 1, 1);
    // e3:SetValue(s.value)
    effect::set_value_fn(d, e3, lua_fn(value));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.target(e,c)`
fn target(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsLevelAbove(6) and c:IsSpecialSummoned()
    card::is_level_above(d, c, 6) && card::is_special_summoned(d, c)
}
/// `s.value(e,re,tp)`
fn value(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    // local rc=re:GetHandler()
    // return re:IsMonsterEffect() and rc:IsOnField() and rc:IsLevelAbove(6) and rc:IsSpecialSummoned()
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    let rc = effect::get_handler(d, re).nil();
    card::is_on_field(d, rc) && card::is_level_above(d, rc, 6) && card::is_special_summoned(d, rc)
}
