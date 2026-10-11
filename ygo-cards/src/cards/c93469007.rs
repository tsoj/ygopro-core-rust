//! Assault Overload
use super::prelude::*;

pub const ID: u32 = 93469007;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ASSAULT_MODE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `Card.IsSetCard` with `SET_ASSAULT_MODE` (the extra argument of the release cost)
fn set_filter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_ASSAULT_MODE as u16])
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,Card.IsSetCard,1,false,nil,nil,SET_ASSAULT_MODE) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&set_filter), 1, 1, false, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,Card.IsSetCard,1,1,false,nil,nil,SET_ASSAULT_MODE)
    let g = duel::select_release_group_cost(d, a.tp, Some(&set_filter), 1, 1, false, None, NO_EX);
    // e:SetLabel(g:GetFirst():GetLevel()*200)
    let first = group::get_first(d, &g).nil();
    let lv = card::get_level(d, first) as i64 * 200;
    effect::set_label(d, a.e, &[lv]);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local res=e:GetLabel()~=0
        let res = effect::get_label(d, a.e) != 0;
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return res
        return res;
    }
    // Duel.SetTargetParam(e:GetLabel())
    let label = effect::get_label(d, a.e);
    duel::set_target_param(d, label as u32);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,PLAYER_ALL,e:GetLabel())
    let label = effect::get_label(d, a.e);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, PLAYER_ALL as u8, label as i32);
    // e:SetLabel(0)
    effect::set_label(d, a.e, &[0]);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local d=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let dmg = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
    // Duel.Damage(1-tp,d,REASON_EFFECT,true)
    duel::damage(d, 1 - tp, dmg, REASON_EFFECT, true, None);
    // Duel.Damage(tp,d,REASON_EFFECT,true)
    duel::damage(d, tp, dmg, REASON_EFFECT, true, None);
    // Duel.RDComplete()
    duel::rd_complete(d);
}
