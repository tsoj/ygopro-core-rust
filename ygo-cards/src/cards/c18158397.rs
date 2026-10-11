//! Natural Disaster
use super::prelude::*;

pub const ID: u32 = 18158397;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CLOUDIAN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // damage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e2, CATEGORY_DAMAGE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(activate)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(tp) and c:IsPreviousControler(tp)
    card::is_controler(d, c, tp) && card::is_previous_controler(d, c, tp)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return (r&(REASON_DESTROY|REASON_EFFECT))==(REASON_DESTROY|REASON_EFFECT) and re and re:GetHandler():IsSetCard(SET_CLOUDIAN)
    //     and eg:IsExists(s.cfilter,1,nil,1-tp)
    if (a.r & (REASON_DESTROY | REASON_EFFECT)) != (REASON_DESTROY | REASON_EFFECT) {
        return false;
    }
    let Some(re) = a.re else { return false };
    let h = effect::get_handler(d, re).nil();
    if !card::is_set_card(d, h, &[SET_CLOUDIAN as u16]) {
        return false;
    }
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, 1 - tp), 1, NO_EX)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return e:GetHandler():IsRelateToEffect(e) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, e).nil();
        return card::is_relate_to_effect(d, h, e);
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - tp);
    // Duel.SetTargetParam(500)
    duel::set_target_param(d, 500);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,500)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, 500);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, e).nil();
    if !card::is_relate_to_effect(d, h, e) {
        return;
    }
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let (p, dmg) = (info[0].int() as u8, info[1].int());
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, p, dmg, REASON_EFFECT, false, None);
}
