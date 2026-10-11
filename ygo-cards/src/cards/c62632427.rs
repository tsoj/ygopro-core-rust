//! Margin Trading
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 62632427;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
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
/// `Card.IsAbleToHand` with the extra argument `p` (the player)
fn able_to_hand_for(d: &mut Duel, c: CardId, p: u8) -> bool {
    card::is_able_to_hand(d, c, Some(p))
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToHand,tp,LOCATION_DECK,0,1,nil,tp)
    //     and Duel.IsExistingMatchingCard(Card.IsAbleToHand,tp,0,LOCATION_DECK,1,nil,1-tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| able_to_hand_for(d, c, tp)), tp, LOCATION_DECK, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| able_to_hand_for(d, c, 1 - tp)), tp, 0, LOCATION_DECK, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,PLAYER_ALL,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, PLAYER_ALL as u8, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g1=Duel.GetMatchingGroup(Card.IsAbleToHand,tp,LOCATION_DECK,0,nil,tp)
    let g1 = duel::get_matching_group(d, Some(&|d, c| able_to_hand_for(d, c, tp)), tp, LOCATION_DECK, 0, NO_EX);
    // local g2=Duel.GetMatchingGroup(Card.IsAbleToHand,tp,0,LOCATION_DECK,nil,1-tp)
    let g2 = duel::get_matching_group(d, Some(&|d, c| able_to_hand_for(d, c, 1 - tp)), tp, 0, LOCATION_DECK, NO_EX);
    // if #g1==0 or #g2==0 then return end
    if g1.is_empty() || g2.is_empty() {
        return;
    }
    // Opponent can discard 1 card to negate this effect
    // if Duel.IsChainDisablable(0) then
    if duel::is_chain_disablable(d, 0) {
        // local g=Duel.GetMatchingGroup(Card.IsDiscardable,tp,0,LOCATION_HAND,nil,REASON_EFFECT)
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_discardable(d, c, Some(REASON_EFFECT), None)), tp, 0, LOCATION_HAND, NO_EX);
        // if #g>0 and Duel.SelectYesNo(1-tp,aux.Stringid(id,1)) then
        if !g.is_empty() && duel::select_yes_no(d, 1 - tp, aux::stringid(ID, 1)) {
            // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_DISCARD)
            duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_DISCARD as u64);
            // local sg=g:Select(1-tp,1,1,nil)
            let sg = group::select(d, &g, 1 - tp, 1, 1, false, NO_EX).nil();
            // Duel.SendtoGrave(sg,REASON_EFFECT|REASON_DISCARD)
            duel::send_to_grave(d, sg, REASON_EFFECT | REASON_DISCARD, None, None);
            // return
            return;
        }
    }
    // Each player looks at the opponent's Deck
    // Duel.ConfirmCards(tp,g2)
    duel::confirm_cards(d, tp, g2.clone());
    // Duel.ConfirmCards(1-tp,g1)
    duel::confirm_cards(d, 1 - tp, g1.clone());
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
    // local hg2=g2:Select(tp,1,1,nil)
    let hg2 = group::select(d, &g2, tp, 1, 1, false, NO_EX).nil();
    // Duel.Hint(HINT_SELECTMSG,1-tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, aux::stringid(ID, 2));
    // local hg1=g1:Select(1-tp,1,1,nil)
    let hg1 = group::select(d, &g1, 1 - tp, 1, 1, false, NO_EX).nil();
    // if #hg1==0 or #hg2==0 then return end
    if hg1.is_empty() || hg2.is_empty() {
        return;
    }
    // Duel.BreakEffect()
    duel::break_effect(d);
    // Duel.SendtoHand(hg1,nil,REASON_EFFECT)
    duel::send_to_hand(d, hg1.clone(), None, REASON_EFFECT, None);
    // Duel.ConfirmCards(tp,hg1)
    duel::confirm_cards(d, tp, hg1);
    // Duel.SendtoHand(hg2,nil,REASON_EFFECT)
    duel::send_to_hand(d, hg2.clone(), None, REASON_EFFECT, None);
    // Duel.ConfirmCards(tp,hg2)
    duel::confirm_cards(d, tp, hg2);
}
