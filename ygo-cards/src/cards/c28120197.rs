//! Canyon
use super::prelude::*;

pub const ID: u32 = 28120197;

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
    // Double Damage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CHANGE_BATTLE_DAMAGE)
    effect::set_code(d, e2, EFFECT_CHANGE_BATTLE_DAMAGE);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // e2:SetCondition(s.dcon1)
    effect::set_condition(d, e2, Some(lua_fn(dcon1)));
    // e2:SetValue(DOUBLE_DAMAGE)
    effect::set_value(d, e2, DOUBLE_DAMAGE as i64);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetTargetRange(0,1)
    effect::set_target_range(d, e3, 0, 1);
    // e3:SetCondition(s.dcon2)
    effect::set_condition(d, e3, Some(lua_fn(dcon2)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.dcon1(e)`
fn dcon1(d: &mut Duel, a: Args) -> bool {
    // local a,d=Duel.GetAttacker(),Duel.GetAttackTarget()
    let atk = duel::get_attacker(d).nil();
    let tgt = duel::get_attack_target(d);
    // return a:GetControler()==e:GetHandlerPlayer() and d and d:IsDefensePos() and d:IsRace(RACE_ROCK)
    card::get_controler(d, atk) == effect::get_handler_player(d, a.e(0))
        && tgt.is_some_and(|t| card::is_defense_pos(d, t) && card::is_race(d, t, RACE_ROCK))
}
/// `s.dcon2(e)`
fn dcon2(d: &mut Duel, a: Args) -> bool {
    // local a,d=Duel.GetAttacker(),Duel.GetAttackTarget()
    let atk = duel::get_attacker(d).nil();
    let tgt = duel::get_attack_target(d);
    // return a:GetControler()==1-e:GetHandlerPlayer() and d and d:IsDefensePos() and d:IsRace(RACE_ROCK)
    card::get_controler(d, atk) == 1 - effect::get_handler_player(d, a.e(0))
        && tgt.is_some_and(|t| card::is_defense_pos(d, t) && card::is_race(d, t, RACE_ROCK))
}
