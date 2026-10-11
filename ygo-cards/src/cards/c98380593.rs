//! Supremacy Berry
use super::prelude::*;

pub const ID: u32 = 98380593;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // recover
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCategory(CATEGORY_RECOVER+CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_RECOVER + CATEGORY_DAMAGE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.rectg)
    effect::set_target(d, e1, Some(cost_target(rectg)));
    // e1:SetOperation(s.recop)
    effect::set_operation(d, e1, Some(operation(recop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.rectg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rectg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return e:IsHasType(EFFECT_TYPE_ACTIVATE) and Duel.GetLP(tp)~=Duel.GetLP(1-tp) end
    if a.chk == Some(0) {
        return effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) && duel::get_lp(d, tp) != duel::get_lp(d, 1 - tp);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, tp);
    // if Duel.GetLP(tp)<Duel.GetLP(1-tp) then
    if duel::get_lp(d, tp) < duel::get_lp(d, 1 - tp) {
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // Duel.SetTargetParam(2000)
        duel::set_target_param(d, 2000);
        // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,2000)
        duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, tp, 2000);
    } else {
        // e:SetLabel(1)
        effect::set_label(d, a.e, &[1]);
        // Duel.SetTargetParam(1000)
        duel::set_target_param(d, 1000);
        // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,tp,1000)
        duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, tp, 1000);
    }
    true
}
/// `s.recop(e,tp,eg,ep,ev,re,r,rp)`
fn recop(d: &mut Duel, a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    let dv = info[1].int();
    // if e:GetLabel()==0 then
    if effect::get_label(d, a.e) == 0 {
        // Duel.Recover(p,d,REASON_EFFECT)
        duel::recover(d, p, dv, REASON_EFFECT, false, None);
    } else {
        // Duel.Damage(p,d,REASON_EFFECT)
        duel::damage(d, p, dv, REASON_EFFECT, false, None);
    }
}
