//! Electromagnetic Shield
use super::prelude::*;

pub const ID: u32 = 91107093;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // indestructable
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
    // e2:SetTarget(s.infilter)
    effect::set_target(d, e2, Some(effect_card_fn(infilter)));
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // self destroy
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCode(EFFECT_SELF_DESTROY)
    effect::set_code(d, e3, EFFECT_SELF_DESTROY);
    // e3:SetCondition(s.descon)
    effect::set_condition(d, e3, Some(lua_fn(descon)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.infilter(e,c)`
fn infilter(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsDefensePos() and c:IsLevelBelow(3) and c:IsRace(RACE_THUNDER)
    card::is_defense_pos(d, c) && card::is_level_below(d, c, 3) && card::is_race(d, c, RACE_THUNDER)
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAttackPos()
    card::is_faceup(d, c) && card::is_attack_pos(d, c)
}
/// `s.descon(e)`
fn descon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // return Duel.IsExistingMatchingCard(s.filter,e:GetHandlerPlayer(),LOCATION_MZONE,0,1,nil)
    let p = effect::get_handler_player(d, e);
    duel::is_existing_matching_card(d, Some(&filter), p, LOCATION_MZONE, 0, 1, NO_EX)
}
