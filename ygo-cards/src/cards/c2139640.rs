//! Amabie
use super::prelude::*;

pub const ID: u32 = 2139640;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Make each player gain 300 LP
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetCategory(CATEGORY_RECOVER)
    effect::set_category(d, e1, CATEGORY_RECOVER);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return Duel.IsPhase(PHASE_MAIN1) and not Duel.CheckPhaseActivity() end)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(function(e,tp,eg,ep,ev,re,r,rp,chk) if chk==0 then return not e:GetHandler():IsPublic() end end)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk) if chk==0 then return true end Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,PLAYER_ALL,300) end)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) Duel.Recover(tp,300,REASON_EFFECT) Duel.Recover(1-tp,300,REASON_EFFECT) end)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_phase(d, PHASE_MAIN1) && !duel::check_phase_activity(d)
}
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return !card::is_public(d, c);
    }
    true
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, PLAYER_ALL as u8, 300);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    duel::recover(d, a.tp, 300, REASON_EFFECT, false, None);
    duel::recover(d, 1 - a.tp, 300, REASON_EFFECT, false, None);
}
