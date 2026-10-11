//! Five Brothers Explosion
use super::prelude::*;

pub const ID: u32 = 70946699;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // recover
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCategory(CATEGORY_RECOVER)
    effect::set_category(d, e1, CATEGORY_RECOVER);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // damage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e2, CATEGORY_DAMAGE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCondition(s.damcon)
    effect::set_condition(d, e2, Some(condition(damcon)));
    // e2:SetTarget(s.damtg)
    effect::set_target(d, e2, Some(cost_target(damtg)));
    // e2:SetOperation(s.damop)
    effect::set_operation(d, e2, Some(operation(damop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, tp);
    // local rec=Duel.GetMatchingGroupCount(s.filter,tp,LOCATION_ONFIELD,0,nil)*500
    let rec = duel::get_matching_group_count(d, Some(&filter), tp, LOCATION_ONFIELD, 0, NO_EX) as u32 * 500;
    // Duel.SetTargetParam(rec)
    duel::set_target_param(d, rec);
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,rec)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, tp, rec as i32);
    true
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_CONTINUOUS) and c:IsFaceup()
    card::is_type(d, c, TYPE_CONTINUOUS) && card::is_faceup(d, c)
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local rec=Duel.GetMatchingGroupCount(s.filter,tp,LOCATION_ONFIELD,0,nil)*500
    let rec = duel::get_matching_group_count(d, Some(&filter), tp, LOCATION_ONFIELD, 0, NO_EX) as i64 * 500;
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // Duel.Recover(p,rec,REASON_EFFECT)
    duel::recover(d, p, rec, REASON_EFFECT, false, None);
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousLocation(LOCATION_ONFIELD) and c:IsPreviousPosition(POS_FACEUP) and c:IsReason(REASON_EFFECT)
    //     and c:GetReasonPlayer()~=tp and c:IsPreviousControler(tp)
    card::is_previous_location(d, c, LOCATION_ONFIELD)
        && card::is_previous_position(d, c, POS_FACEUP)
        && card::is_reason(d, c, REASON_EFFECT)
        && card::get_reason_player(d, c) != tp
        && card::is_previous_controler(d, c, tp)
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - tp);
    // local dam=Duel.GetMatchingGroupCount(s.filter,tp,LOCATION_GRAVE,0,nil)*500
    let dam = duel::get_matching_group_count(d, Some(&filter), tp, LOCATION_GRAVE, 0, NO_EX) as u32 * 500;
    // Duel.SetTargetParam(dam)
    duel::set_target_param(d, dam);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,500)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, 500);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local dam=Duel.GetMatchingGroupCount(s.filter,tp,LOCATION_GRAVE,0,nil)*500
    let dam = duel::get_matching_group_count(d, Some(&filter), tp, LOCATION_GRAVE, 0, NO_EX) as i64 * 500;
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // Duel.Damage(p,dam,REASON_EFFECT)
    duel::damage(d, p, dam, REASON_EFFECT, false, None);
}
