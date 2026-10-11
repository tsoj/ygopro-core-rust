//! Photon Veil
use super::prelude::*;

pub const ID: u32 = 9354555;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TODECK+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_TOHAND + CATEGORY_SEARCH);
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
/// `s.tdfilter(c)`: `return c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsAbleToDeck()`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_able_to_deck(d, c)
}
/// `s.thfilter(c)`: `return c:IsLevelBelow(4) and c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsAbleToHand()`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_level_below(d, c, 4) && card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tdfilter,tp,LOCATION_HAND,0,3,nil)
    //     and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tdfilter), tp, LOCATION_HAND, 0, 3, NO_EX)
            && duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,3,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 3, tp, LOCATION_HAND as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.rescon(sg,e,tp,mg)`: `return #sg<2 or sg:GetClassCount(Card.GetCode)==1`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    let res = sg.len() < 2 || group::get_class_count(d, sg, &|d, c| card::get_code(d, c)[0] as i64) == 1;
    (res, false)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectMatchingCard(tp,s.tdfilter,tp,LOCATION_HAND,0,3,3,nil)
    let g = duel::select_matching_card(d, tp, Some(&tdfilter), tp, LOCATION_HAND, 0, 3, 3, false, NO_EX).nil();
    // if #g<3 then return end
    if g.len() < 3 {
        return;
    }
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - tp, g.clone());
    // if Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)~=3
    //     or not g:IsExists(Card.IsLocation,3,nil,LOCATION_DECK) then return end
    if duel::send_to_deck(d, g.clone(), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) != 3
        || !group::is_exists(d, &g, &|d, c| card::is_location(d, c, LOCATION_DECK), 3, NO_EX)
    {
        return;
    }
    // local sg=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
    let sg = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
    // if #sg==0 then return end
    if sg.is_empty() {
        return;
    }
    // local hg=aux.SelectUnselectGroup(sg,e,tp,1,3,s.rescon,1,tp,HINTMSG_ATOHAND)
    let hg = aux::select_unselect_group(d, &sg, Some(e), tp, Some(1), Some(3), Some(&rescon), 1, tp, Some(HINTMSG_ATOHAND as u64), None, None, false).unwrap_or_default();
    // if #hg>0 then
    if !hg.is_empty() {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SendtoHand(hg,nil,REASON_EFFECT)
        duel::send_to_hand(d, hg.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,hg)
        duel::confirm_cards(d, 1 - tp, hg);
    }
}
