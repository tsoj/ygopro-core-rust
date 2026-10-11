//! Diced Dice
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 93078761;

pub const INFO: ScriptInfo = ScriptInfo { roll_dice: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DICE+CATEGORY_TOHAND+CATEGORY_TODECK+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_DICE + CATEGORY_TOHAND + CATEGORY_TODECK + CATEGORY_SEARCH);
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
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c.roll_dice and c:IsAbleToHand()
    crate::card_info(d, c).roll_dice && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DICE,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DICE, None, 0, a.tp, 1);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local r1=Duel.TossDice(tp,1)
    let r1 = duel::toss_dice(d, tp, 1, 0)[0];
    // if r1==1 or r1==6 then
    if r1 == 1 || r1 == 6 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&filter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - tp, g);
        }
    // elseif c:IsRelateToEffect(e) then
    } else if card::is_relate_to_effect(d, c, a.e) {
        // c:CancelToGrave()
        card::cancel_to_grave(d, c, true);
        // local r2=Duel.TossDice(tp,1)
        let r2 = duel::toss_dice(d, tp, 1, 0)[0];
        // if r2==1 or r2==6 then
        if r2 == 1 || r2 == 6 {
            // Duel.SendtoHand(c,nil,REASON_EFFECT)
            duel::send_to_hand(d, c, None, REASON_EFFECT, None);
        } else {
            // Duel.SendtoDeck(c,nil,SEQ_DECKTOP,REASON_EFFECT)
            duel::send_to_deck(d, c, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
        }
    }
}
