//! Return Soul
//! リターンソウル
use super::prelude::*;

pub const ID: u32 = 10537981;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_TODECK);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, TIMING_END_PHASE, None);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsPhase(PHASE_END)
    duel::is_phase(d, PHASE_END)
}
/// `s.filter(c,tid)`
fn filter(d: &mut Duel, c: CardId, tid: u32) -> bool {
    // return c:IsReason(REASON_DESTROY) and c:IsMonster() and c:GetTurnID()==tid and c:IsAbleToDeck()
    card::is_reason(d, c, REASON_DESTROY) && card::is_monster(d, c) && card::get_turn_id(d, c) == tid && card::is_able_to_deck(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and s.filter(chkc,Duel.GetTurnCount()) end
    if let Some(chkc) = a.chkc() {
        let tid = duel::get_turn_count(d, None) as u32;
        return card::is_location(d, chkc, LOCATION_GRAVE) && filter(d, chkc, tid);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,nil,Duel.GetTurnCount()) end
    if a.chk == Some(0) {
        let tid = duel::get_turn_count(d, None) as u32;
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, tid)), tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,3,nil,Duel.GetTurnCount())
    let tid = duel::get_turn_count(d, None) as u32;
    let g = duel::select_target(d, tp, Some(&|d, c| filter(d, c, tid)), tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, 3, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local sg=Duel.GetTargetCards(e)
    let sg = duel::get_target_cards(d, a.e).nil();
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.SendtoDeck(sg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, sg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
