//! Xyz Gift
use super::prelude::*;

pub const ID: u32 = 72355441;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_XYZ)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,2,nil)
    duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 2, NO_EX)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,2) and Duel.CheckRemoveOverlayCard(tp,1,0,2,REASON_EFFECT) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, 2) && duel::check_remove_overlay_card(d, tp, 1, 0, 2, REASON_EFFECT, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,2)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 2);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not Duel.CheckRemoveOverlayCard(tp,1,0,2,REASON_EFFECT) then return end
    if !duel::check_remove_overlay_card(d, tp, 1, 0, 2, REASON_EFFECT, None) {
        return;
    }
    // Duel.RemoveOverlayCard(tp,1,0,2,2,REASON_EFFECT)
    duel::remove_overlay_card(d, tp, 1, 0, 2, 2, REASON_EFFECT, None);
    // Duel.Draw(tp,2,REASON_EFFECT)
    duel::draw(d, tp, 2, REASON_EFFECT);
}
