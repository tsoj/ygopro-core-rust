//! Old Mind
use super::prelude::*;

pub const ID: u32 = 54239282;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Look at 1 random card in your opponent's hand, then apply 1 of the following effects.
    // ● Discard both that opponent's card and 1 card from your hand of the same type (Monster, Spell, or Trap), then add this card from the field to your opponent's hand, and if you do, draw 1 card.
    // ● Lose 1000 LP.
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_HANDES+CATEGORY_DRAW+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_HANDES + CATEGORY_DRAW + CATEGORY_TOHAND);
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
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.NOT(Card.IsPublic),tp,0,LOCATION_HAND,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| !card::is_public(d, c)), tp, 0, LOCATION_HAND, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,1,PLAYER_ALL,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 1, PLAYER_ALL as u8, LOCATION_HAND as i32);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    true
}
// function s.filter(c,typ)
fn filter(d: &mut Duel, c: CardId, typ: u32) -> bool {
    // return c:IsType(typ) and c:IsDiscardable(REASON_EFFECT)
    card::is_type(d, c, typ) && card::is_discardable(d, c, Some(REASON_EFFECT), None)
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetMatchingGroup(aux.NOT(Card.IsPublic),tp,0,LOCATION_HAND,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| !card::is_public(d, c)), tp, 0, LOCATION_HAND, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local oc=g:RandomSelect(tp,1):GetFirst()
    let picked = group::random_select(d, &g, tp, 1);
    let oc = group::get_first(d, &picked).nil();
    // Duel.ConfirmCards(tp,oc)
    duel::confirm_cards(d, tp, CardSet::from_card(oc));
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local b1=Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND,0,1,nil,oc:GetMainCardType())
    //     and Duel.IsPlayerCanDraw(tp,1) and c:IsRelateToEffect(e)
    let oct = card::get_main_card_type(d, oc);
    let b1 = duel::is_existing_matching_card(d, Some(&|d, x| filter(d, x, oct)), tp, LOCATION_HAND, 0, 1, NO_EX)
        && duel::is_player_can_draw(d, tp, 1)
        && card::is_relate_to_effect(d, c, e);
    // local b2=true
    let b2 = true;
    // local option=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,0)},
    //     {b2,aux.Stringid(id,1)})
    let option = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 0)), (b2, aux::stringid(ID, 1))]);
    // if option==1 then
    if option == Some(1) {
        // --Discard both that opponent's card and 1 card from your hand of the same type (Monster, Spell, or Trap), then add this card from the field to your opponent's hand, and if you do, draw 1 card
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DISCARD)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DISCARD as u64);
        // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_HAND,0,1,1,nil,oc:GetMainCardType())
        let mut g = duel::select_matching_card(d, tp, Some(&|d, x| filter(d, x, oct)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        // g:AddCard(oc)
        group::add_card(&mut g, &CardSet::from_card(oc));
        // if Duel.SendtoGrave(g,REASON_EFFECT|REASON_DISCARD)==2 then
        if duel::send_to_grave(d, g, REASON_EFFECT | REASON_DISCARD, None, None) == 2 {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // c:CancelToGrave()
            card::cancel_to_grave(d, c, true);
            // Duel.SendtoHand(c,1-tp,REASON_EFFECT)
            duel::send_to_hand(d, CardSet::from_card(c), Some(1 - tp), REASON_EFFECT, None);
            // Duel.Draw(tp,1,REASON_EFFECT)
            duel::draw(d, tp, 1, REASON_EFFECT);
        }
    } else if option == Some(2) {
        // --● Lose 1000 LP
        // Duel.SetLP(tp,math.max(Duel.GetLP(tp)-1000,0))
        let lp = (duel::get_lp(d, tp) - 1000).max(0);
        duel::set_lp(d, tp, lp);
    }
    // Duel.ShuffleHand(1-tp)
    duel::shuffle_hand(d, 1 - tp);
}
