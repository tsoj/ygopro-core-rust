//! The Legend of Tickets
//! Ｔｈｅ Ｌｅｇｅｎｄ ｏｆ Ｔｉｃｋｅｔｓ
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 54120521;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_TODECK);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE);
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, tp, None, tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TODECK,g,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_TODECK, Some(g), 1, tp, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Place it on top of the Deck during the End Phase of this turn
        // aux.DelayedOperation(tc,PHASE_END,id,e,tp,function(cc) Duel.HintSelection(cc,true) Duel.SendtoDeck(cc,nil,SEQ_DECKTOP,REASON_EFFECT) end)
        aux::delayed_operation(
            d,
            tc,
            PHASE_END,
            ID,
            a.e,
            tp,
            Some(std::sync::Arc::new(delayed_send_to_deck)),
            None,
            None,
            None,
            None,
            None,
        );
    }
    // local turn_ct=Duel.GetTurnCount()
    let turn_ct = duel::get_turn_count(d, None);
    // local ct=(Duel.IsTurnPlayer(tp) and Duel.IsPhase(PHASE_DRAW)) and 2 or 1
    let ct: u16 = if duel::is_turn_player(d, tp) && duel::is_phase(d, PHASE_DRAW) { 2 } else { 1 };
    // Draw 2 cards instead of 1 for your normal draw during your next Draw Phase
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_DRAW_COUNT)
    effect::set_code(d, e1, EFFECT_DRAW_COUNT);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetCondition(function() return Duel.GetTurnCount()~=turn_ct end)
    effect::set_condition(d, e1, Some(condition(move |d, _a| duel::get_turn_count(d, None) != turn_ct)));
    // e1:SetValue(2)
    effect::set_value(d, e1, 2);
    // e1:SetReset(RESET_PHASE|PHASE_DRAW|RESET_SELF_TURN,ct)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_DRAW | RESET_SELF_TURN, ct);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// `function(cc) Duel.HintSelection(cc,true) Duel.SendtoDeck(cc,nil,SEQ_DECKTOP,REASON_EFFECT) end`
/// (the DelayedOperation callback: the cards are the first argument)
fn delayed_send_to_deck(d: &mut Duel, cc: &CardSet, _a: &EffArgs) -> bool {
    // Duel.HintSelection(cc,true)
    duel::hint_selection(d, cc.clone(), true);
    // Duel.SendtoDeck(cc,nil,SEQ_DECKTOP,REASON_EFFECT)
    duel::send_to_deck(d, cc.clone(), None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
    true
}
