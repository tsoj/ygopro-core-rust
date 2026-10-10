//! Pharaoh's Treasure
use super::prelude::*;

pub const ID: u32 = 63571750;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Duel.EnableGlobalFlag(GLOBALFLAG_DECK_REVERSE_CHECK)
    duel::enable_global_flag(d, GLOBALFLAG_DECK_REVERSE_CHECK);
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_TODECK);
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
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsAbleToDeck() end
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_deck(d, h);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // c:CancelToGrave()
    card::cancel_to_grave(d, c, true);
    // if Duel.SendtoDeck(c,tp,SEQ_DECKSHUFFLE,REASON_EFFECT)~=0 and c:IsLocation(LOCATION_DECK) then
    if duel::send_to_deck(d, c, Some(tp), SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) != 0 && card::is_location(d, c, LOCATION_DECK) {
        // c:ReverseInDeck()
        card::reverse_in_deck(d, c);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,0))
        effect::set_description(d, e1, aux::stringid(ID, 0));
        // e1:SetCategory(CATEGORY_TOHAND)
        effect::set_category(d, e1, CATEGORY_TOHAND);
        // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
        // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
        // e1:SetCode(EVENT_DRAW)
        effect::set_code(d, e1, EVENT_DRAW);
        // e1:SetTarget(s.thtg)
        effect::set_target(d, e1, Some(cost_target(thtg)));
        // e1:SetOperation(s.thop)
        effect::set_operation(d, e1, Some(operation(thop)));
        // e1:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOHAND))
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TOHAND), 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and chkc:IsAbleToHand() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && card::is_able_to_hand(d, chkc, None);
    }
    // if chk==0 then return e:GetHandler():IsRelateToEffect(e) and Duel.IsExistingTarget(Card.IsAbleToHand,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_relate_to_effect(d, h, a.e)
            && duel::is_existing_target(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local sg=Duel.SelectTarget(tp,Card.IsAbleToHand,tp,LOCATION_GRAVE,0,1,1,nil)
    let sg = duel::select_target(d, tp, Some(&|d, c| card::is_able_to_hand(d, c, None)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,sg,#sg,0,0)
    let n = sg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(sg), n, 0, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and c:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && card::is_relate_to_effect(d, c, a.e) {
            // if Duel.SendtoGrave(c,REASON_EFFECT)~=0 and c:IsLocation(LOCATION_GRAVE) then
            if duel::send_to_grave(d, c, REASON_EFFECT, None, None) != 0 && card::is_location(d, c, LOCATION_GRAVE) {
                // Duel.SendtoHand(tc,nil,REASON_EFFECT)
                duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
                // Duel.ConfirmCards(1-tp,tc)
                duel::confirm_cards(d, 1 - tp, tc);
            }
        }
    }
}
