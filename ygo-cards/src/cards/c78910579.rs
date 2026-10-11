//! Aquamirror Cycle
use super::prelude::*;

pub const ID: u32 = 78910579;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Target 1 WATER monster you control and 2 WATER monsters in your Graveyard; shuffle the first target into the Deck, and if you do, add the second targets to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.filter1(c)
fn filter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAttribute(ATTRIBUTE_WATER) and c:IsAbleToDeck()
    card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_WATER as u32) && card::is_able_to_deck(d, c)
}
// function s.filter2(c)
fn filter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_WATER) and c:IsAbleToHand()
    card::is_attribute(d, c, ATTRIBUTE_WATER as u32) && card::is_able_to_hand(d, c, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter1,tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingTarget(s.filter2,tp,LOCATION_GRAVE,0,2,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter1), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&filter2), a.tp, LOCATION_GRAVE, 0, 2, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g1=Duel.SelectTarget(tp,s.filter1,tp,LOCATION_MZONE,0,1,1,nil)
    let g1 = duel::select_target(d, a.tp, Some(&filter1), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    // local g2=Duel.SelectTarget(tp,s.filter2,tp,LOCATION_GRAVE,0,2,2,nil)
    let g2 = duel::select_target(d, a.tp, Some(&filter2), a.tp, LOCATION_GRAVE, 0, 2, 2, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g1,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, g1, 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g2,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g2, 2, a.tp, 0);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local ex,g1=Duel.GetOperationInfo(0,CATEGORY_TODECK)
    let g1 = duel::get_operation_info(d, 0, CATEGORY_TODECK).and_then(|o| o.op_cards).nil();
    // local ex,g2=Duel.GetOperationInfo(0,CATEGORY_TOHAND)
    let g2 = duel::get_operation_info(d, 0, CATEGORY_TOHAND).and_then(|o| o.op_cards).nil();
    // local tc1=g1:GetFirst()
    let tc1 = group::get_first(d, &g1).nil();
    // if tc1:IsRelateToEffect(e) and Duel.SendtoDeck(tc1,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)>=0 and tc1:IsLocation(LOCATION_DECK|LOCATION_EXTRA) then
    if card::is_relate_to_effect(d, tc1, e)
        && duel::send_to_deck(d, tc1, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) >= 0
        && card::is_location(d, tc1, LOCATION_DECK | LOCATION_EXTRA)
    {
        // local hg=g2:Filter(Card.IsRelateToEffect,nil,e)
        let hg = group::filter(d, &g2, &|d, c| card::is_relate_to_effect(d, c, e), NO_EX);
        // Duel.SendtoHand(hg,nil,REASON_EFFECT)
        duel::send_to_hand(d, hg.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,hg)
        duel::confirm_cards(d, 1 - tp, hg);
    }
}
