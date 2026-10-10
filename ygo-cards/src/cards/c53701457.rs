//! No.28 Titanic Moth
use super::prelude::*;

pub const ID: u32 = 53701457;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Xyz summon
    // Xyz.AddProcedure(c,nil,7,2)
    xyz::add_procedure(d, c, None, Some(7), 2, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Direct attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
    // e1:SetCondition(s.dircon)
    effect::set_condition(d, e1, Some(condition(dircon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Damage reduce
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_CHANGE_BATTLE_DAMAGE)
    effect::set_code(d, e2, EFFECT_CHANGE_BATTLE_DAMAGE);
    // e2:SetCondition(s.rdcon)
    effect::set_condition(d, e2, Some(condition(rdcon)));
    // e2:SetValue(aux.ChangeBattleDamage(1,HALF_DAMAGE))
    effect::set_value_fn(d, e2, aux::change_battle_damage(1, HALF_DAMAGE as i64));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Inflict damage
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e3, CATEGORY_DAMAGE);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e3, EVENT_BATTLE_DAMAGE);
    // e3:SetCondition(s.damcon)
    effect::set_condition(d, e3, Some(condition(damcon)));
    // e3:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e3, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e3:SetTarget(s.damtg)
    effect::set_target(d, e3, Some(cost_target(damtg)));
    // e3:SetOperation(s.damop)
    effect::set_operation(d, e3, Some(operation(damop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.dircon(e)`
fn dircon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(e:GetHandlerPlayer(),LOCATION_MZONE,0)<=1
    let p = effect::get_handler_player(d, a.e);
    duel::get_field_group_count(d, p, LOCATION_MZONE, 0) <= 1
}
/// `s.rdcon(e,tp,eg,ep,ev,re,r,rp)`
fn rdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c,tp=e:GetHandler(),e:GetHandlerPlayer()
    let c = effect::get_handler(d, a.e).nil();
    let tp = effect::get_handler_player(d, a.e);
    // return Duel.GetAttackTarget()==nil and c:GetEffectCount(EFFECT_DIRECT_ATTACK)<2 and Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>0
    duel::get_attack_target(d).is_none()
        && card::get_effect_count(d, c, EFFECT_DIRECT_ATTACK) < 2
        && duel::get_field_group_count(d, tp, 0, LOCATION_MZONE) > 0
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(_d: &mut Duel, a: &EffArgs) -> bool {
    // return ep~=tp
    a.ep != a.tp
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ct=Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)
    let ct = duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND);
    // if chk==0 then return ct>0 end
    if a.chk == Some(0) {
        return ct > 0;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // local dam=ct*500
    let dam = ct as i64 * 500;
    // Duel.SetTargetParam(dam)
    duel::set_target_param(d, dam as u32);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,dam)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, dam as i32);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, _a: &EffArgs) {
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // local dam=Duel.GetFieldGroupCount(p,LOCATION_HAND,0)*500
    let dam = duel::get_field_group_count(d, p, LOCATION_HAND, 0) as i64 * 500;
    // Duel.Damage(p,dam,REASON_EFFECT)
    duel::damage(d, p, dam, REASON_EFFECT, false, None);
}
