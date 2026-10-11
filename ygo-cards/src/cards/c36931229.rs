//! Castle Gate
use super::prelude::*;

pub const ID: u32 = 36931229;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Cannot be destroyed by battle
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Tribute 1 Level 5 or lower monster to inflict damage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e2, CATEGORY_DAMAGE);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(function(e) return e:GetHandler():IsAttackPos() end)
    effect::set_condition(d, e2, Some(condition(condition_fn)));
    // e2:SetCost(s.cost)
    effect::set_cost(d, e2, Some(cost_target(cost_fn)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(operation_fn)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function(e) return e:GetHandler():IsAttackPos() end
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_attack_pos(d, h)
}
// s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelBelow(5) and c:GetTextAttack()>0
    card::is_level_below(d, c, 5) && card::get_text_attack(d, c) > 0
}
// s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.cfilter,1,false,nil,nil) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&|d, c| cfilter(d, c)), 1, 1, false, None, NO_EX);
    }
    // local sg=Duel.SelectReleaseGroupCost(tp,s.cfilter,1,1,false,nil,nil)
    let sg = duel::select_release_group_cost(d, a.tp, Some(&|d, c| cfilter(d, c)), 1, 1, false, None, NO_EX);
    // e:SetLabel(sg:GetFirst():GetTextAttack())
    let first = group::get_first(d, &sg).nil();
    let atk = card::get_text_attack(d, first) as i64;
    effect::set_label(d, a.e, &[atk]);
    // Duel.Release(sg,REASON_COST)
    duel::release(d, sg, REASON_COST, None);
    true
}
// s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(e:GetLabel())
    let label = effect::get_label(d, a.e);
    duel::set_target_param(d, label as u32);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,e:GetLabel())
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, label as i32);
    true
}
// s.operation(e,tp,eg,ep,ev,re,r,rp)
fn operation_fn(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
