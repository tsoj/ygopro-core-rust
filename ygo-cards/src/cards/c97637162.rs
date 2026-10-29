//! Handigallop
use super::prelude::*;

pub const ID: u32 = 97637162;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot attack direct
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CANNOT_DIRECT_ATTACK)
    effect::set_code(d, e1, EFFECT_CANNOT_DIRECT_ATTACK);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // gains attack
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(s.atkval)
    effect::set_value_fn(d, e2, effect_value(atkval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // battle damage change
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_REFLECT_BATTLE_DAMAGE)
    effect::set_code(d, e3, EFFECT_REFLECT_BATTLE_DAMAGE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetTargetRange(0,1)
    effect::set_target_range(d, e3, 0, 1);
    // e3:SetCondition(s.dmgcond)
    effect::set_condition(d, e3, Some(condition(dmgcond)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId) -> i64 {
    // return math.abs(Duel.GetLP(1)-Duel.GetLP(0))
    (duel::get_lp(d, 1) - duel::get_lp(d, 0)).abs() as i64
}
/// `s.dmgcond(e)`
fn dmgcond(d: &mut Duel, a: &EffArgs) -> bool {
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e);
    // return Duel.GetLP(tp)>Duel.GetLP(1-tp) and Duel.GetAttacker()==e:GetHandler()
    duel::get_lp(d, tp) > duel::get_lp(d, 1 - tp) && duel::get_attacker(d) == Some(effect::get_handler(d, a.e).nil())
}
