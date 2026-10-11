//! Worm Hope
use super::prelude::*;

pub const ID: u32 = 11159464;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // flip
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FLIP+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_FLIP + EFFECT_TYPE_SINGLE);
    // e1:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DRAW);
    // e1:SetTarget(s.drtg)
    effect::set_target(d, e1, Some(cost_target(drtg)));
    // e1:SetOperation(s.drop)
    effect::set_operation(d, e1, Some(operation(drop_op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);

    // discard
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_HANDES)
    effect::set_category(d, e2, CATEGORY_HANDES);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCondition(s.tgcon)
    effect::set_condition(d, e2, Some(condition(tgcon)));
    // e2:SetTarget(s.tgtg)
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    // e2:SetOperation(s.tgop)
    effect::set_operation(d, e2, Some(operation(tgop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // if Duel.IsPhase(PHASE_DAMAGE) and e:GetHandler()==Duel.GetAttackTarget() then
    if duel::is_phase(d, PHASE_DAMAGE) && effect::get_handler(d, a.e) == duel::get_attack_target(d) {
        // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    }
    true
}
// function s.drop(e,tp,eg,ep,ev,re,r,rp)
fn drop_op(d: &mut Duel, a: &EffArgs) {
    // if Duel.IsPhase(PHASE_DAMAGE) and e:GetHandler()==Duel.GetAttackTarget() then
    if duel::is_phase(d, PHASE_DAMAGE) && effect::get_handler(d, a.e) == duel::get_attack_target(d) {
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, a.tp, 1, REASON_EFFECT);
    }
}
// function s.tgcon(e,tp,eg,ep,ev,re,r,rp)
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return (e:GetHandler():GetPreviousLocation()&LOCATION_ONFIELD)~=0
    let h = effect::get_handler(d, a.e).nil();
    (card::get_previous_location(d, h) & LOCATION_ONFIELD) != 0
}
// function s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
// function s.tgop(e,tp,eg,ep,ev,re,r,rp)
fn tgop(d: &mut Duel, a: &EffArgs) {
    // Duel.DiscardHand(tp,nil,1,1,REASON_EFFECT,nil)
    duel::discard_hand(d, a.tp, None, 1, 1, REASON_EFFECT, NO_EX);
}
