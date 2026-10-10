//! Gravekeeper's Cannonholder
use super::prelude::*;

pub const ID: u32 = 99877698;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GRAVEKEEPERS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // damage
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.costfilter(c)`
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_GRAVEKEEPERS) and c:GetCode()~=id
    card::is_set_card(d, c, &[SET_GRAVEKEEPERS as u16]) && card::get_code(d, c)[0] != ID
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.costfilter,1,false,nil,nil) end
    if a.chk == Some(0) {
        return aux::check_release_group_cost(d, a.tp, Some(&costfilter), 1, 1, false, None, NO_EX);
    }
    // local sg=Duel.SelectReleaseGroupCost(tp,s.costfilter,1,1,false,nil,nil)
    let sg = aux::select_release_group_cost(d, a.tp, Some(&costfilter), 1, 1, false, None, NO_EX);
    // Duel.Release(sg,REASON_COST)
    duel::release(d, sg, REASON_COST, None);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(700)
    duel::set_target_param(d, 700);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,700)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 700);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp,chk)`
fn op(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
