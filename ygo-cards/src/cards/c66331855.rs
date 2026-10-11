//! D・チャッカン (Morphtronic Datatron)
use super::prelude::*;

pub const ID: u32 = 66331855;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // damagea
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.cona)
    effect::set_condition(d, e1, Some(condition(cona)));
    // e1:SetCost(s.costa)
    effect::set_cost(d, e1, Some(cost_target(costa)));
    // e1:SetTarget(s.tga)
    effect::set_target(d, e1, Some(cost_target(tga)));
    // e1:SetOperation(s.op)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // damaged
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e2, CATEGORY_DAMAGE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.cond)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetTarget(s.tgd)
    effect::set_target(d, e2, Some(cost_target(tgd)));
    // e2:SetOperation(s.op)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.cona(e,tp,eg,ep,ev,re,r,rp)
fn cona(d: &mut Duel, a: &EffArgs) -> bool {
    // return not e:GetHandler():IsDisabled() and e:GetHandler():IsAttackPos()
    let handler = effect::get_handler(d, a.e).nil();
    !card::is_disabled(d, handler) && card::is_attack_pos(d, handler)
}
// function s.cond(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return not e:GetHandler():IsDisabled() and e:GetHandler():IsDefensePos()
    let handler = effect::get_handler(d, a.e).nil();
    !card::is_disabled(d, handler) && card::is_defense_pos(d, handler)
}
// function s.costa(e,tp,eg,ep,ev,re,r,rp,chk)
fn costa(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,aux.TRUE,1,false,nil,nil) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&|_d: &mut Duel, _c: CardId| true), 1, 1, false, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,aux.TRUE,1,1,false,nil,nil)
    let g = duel::select_release_group_cost(d, a.tp, Some(&|_d: &mut Duel, _c: CardId| true), 1, 1, false, None, NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
// function s.tga(e,tp,eg,ep,ev,re,r,rp,chk)
fn tga(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(600)
    duel::set_target_param(d, 600);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,600)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 600);
    true
}
// function s.tgd(e,tp,eg,ep,ev,re,r,rp,chk)
fn tgd(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(300)
    duel::set_target_param(d, 300);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,300)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 300);
    true
}
// function s.op(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
