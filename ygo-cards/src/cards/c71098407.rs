//! Memory Loss
use super::prelude::*;

pub const ID: u32 = 71098407;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DISABLE+CATEGORY_POSITION)
    effect::set_category(d, e1, CATEGORY_DISABLE + CATEGORY_POSITION);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp~=tp and re:IsMonsterEffect() and Duel.IsChainDisablable(ev)
    if a.rp == a.tp {
        return false;
    }
    let re = a.re.nil();
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    if !duel::is_chain_disablable(d, a.ev as u8) {
        return false;
    }
    // and re:GetHandler():IsLocation(LOCATION_MZONE) and re:GetHandler():IsPosition(POS_FACEUP_ATTACK)
    let rc = effect::get_handler(d, re).nil();
    if !card::is_location(d, rc, LOCATION_MZONE) {
        return false;
    }
    if !card::is_position(d, rc, POS_FACEUP_ATTACK) {
        return false;
    }
    // and re:GetHandler():IsCanChangePosition()
    card::is_can_change_position(d, rc)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetCard(re:GetHandler())
    let rc = effect::get_handler(d, a.re.nil()).nil();
    duel::set_target_card(d, rc);
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, 0, 0);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, a.re.nil()).nil();
    // if Duel.NegateEffect(ev) and rc:IsRelateToEffect(e) then
    if duel::negate_effect(d, a.ev as u8) && card::is_relate_to_effect(d, rc, a.e) {
        // Duel.ChangePosition(rc,POS_FACEUP_DEFENSE)
        duel::change_position(d, rc, POS_FACEUP_DEFENSE as u8, None, None, None, false);
    }
}
