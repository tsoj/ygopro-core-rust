//! Magical Contract Door
use super::prelude::*;

pub const ID: u32 = 18809562;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
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
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return (c:GetLevel()==7 or c:GetLevel()==8) and c:IsAttribute(ATTRIBUTE_DARK) and c:IsAbleToHand()
    let lv = card::get_level(d, c);
    (lv == 7 || lv == 8) && card::is_attribute(d, c, ATTRIBUTE_DARK) && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsSpell,tp,LOCATION_HAND,0,1,e:GetHandler())
    //   and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&card::is_spell), tp, LOCATION_HAND, 0, 1, c.into())
            && duel::is_existing_matching_card(d, Some(&filter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,0))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 0));
    // local ag=Duel.SelectMatchingCard(tp,Card.IsSpell,tp,LOCATION_HAND,0,1,1,nil)
    let ag = duel::select_matching_card(d, tp, Some(&card::is_spell), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #ag>0 then
    if !ag.is_empty() {
        // Duel.SendtoHand(ag,1-tp,REASON_EFFECT)
        duel::send_to_hand(d, ag.clone(), Some(1 - tp), REASON_EFFECT, None);
        // Duel.ConfirmCards(tp,ag)
        duel::confirm_cards(d, tp, ag);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // Duel.ShuffleHand(1-tp)
        duel::shuffle_hand(d, 1 - tp);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&filter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - tp, g);
        }
    }
}
