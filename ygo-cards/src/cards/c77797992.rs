//! Alpacaribou, Mystical Beast of the Forest
use super::prelude::*;

pub const ID: u32 = 77797992;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // indes1
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetCondition(s.cona)
    effect::set_condition(d, e1, Some(lua_fn(cona)));
    // e1:SetTarget(s.targeta)
    effect::set_target(d, e1, Some(effect_card_fn(targeta)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // cannot be target
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetCondition(s.cond)
    effect::set_condition(d, e2, Some(lua_fn(cond)));
    // e2:SetTarget(s.targetd)
    effect::set_target(d, e2, Some(effect_card_fn(targetd)));
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e3, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e3:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e3, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.cona(e)`
fn cona(d: &mut Duel, a: Args) -> bool {
    // return e:GetHandler():IsAttackPos()
    let h = effect::get_handler(d, a.e(0)).nil();
    card::is_attack_pos(d, h)
}
/// `s.targeta(e,c)`
fn targeta(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsPosition(POS_FACEUP_ATTACK) and c:IsRace(RACE_WINGEDBEAST|RACE_PLANT|RACE_INSECT)
    card::is_position(d, c, POS_FACEUP_ATTACK) && card::is_race(d, c, RACE_WINGEDBEAST | RACE_PLANT | RACE_INSECT)
}
/// `s.cond(e)`
fn cond(d: &mut Duel, a: Args) -> bool {
    // return e:GetHandler():IsDefensePos()
    let h = effect::get_handler(d, a.e(0)).nil();
    card::is_defense_pos(d, h)
}
/// `s.targetd(e,c)`
fn targetd(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsPosition(POS_FACEUP_DEFENSE) and c:IsRace(RACE_WINGEDBEAST|RACE_PLANT|RACE_INSECT)
    card::is_position(d, c, POS_FACEUP_DEFENSE) && card::is_race(d, c, RACE_WINGEDBEAST | RACE_PLANT | RACE_INSECT)
}
