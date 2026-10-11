//! Performapal Gongato
use super::prelude::*;

pub const ID: u32 = 9106362;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Negate damage (direct)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.dmcon1)
    effect::set_condition(d, e1, Some(condition(dmcon1)));
    // e1:SetOperation(s.dmop)
    effect::set_operation(d, e1, Some(operation(dmop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate damage (monster)
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.dmcon2)
    effect::set_condition(d, e2, Some(condition(dmcon2)));
    // e2:SetOperation(s.dmop)
    effect::set_operation(d, e2, Some(operation(dmop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.dmcon1(e,tp,eg,ep,ev,re,r,rp)`
fn dmcon1(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetAttacker():IsControler(1-tp) and Duel.GetAttackTarget()==nil
    let attacker = duel::get_attacker(d).nil();
    card::is_controler(d, attacker, 1 - a.tp) && duel::get_attack_target(d).is_none()
}
/// `s.dmcon2(e,tp,eg,ep,ev,re,r,rp)`
fn dmcon2(d: &mut Duel, _a: &EffArgs) -> bool {
    // local a=Duel.GetAttacker()
    let attacker = duel::get_attacker(d);
    // local d=Duel.GetAttackTarget()
    // return d and a:GetControler()~=d:GetControler()
    let Some(target) = duel::get_attack_target(d) else {
        return false;
    };
    let attacker = attacker.nil();
    card::get_controler(d, attacker) != card::get_controler(d, target)
}
/// `s.dmop(e,tp,eg,ep,ev,re,r,rp)`
fn dmop(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_AVOID_BATTLE_DAMAGE)
    effect::set_code(d, e1, EFFECT_AVOID_BATTLE_DAMAGE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetReset(RESET_PHASE|PHASE_DAMAGE)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
