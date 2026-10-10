//! A Cat of Ill Omen
use super::prelude::*;

pub const ID: u32 = 24140059;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_NECROVALLEY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // flip
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_FLIP)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP);
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 1));
    // local g=Duel.SelectMatchingCard(tp,Card.IsTrap,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&card::is_trap), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // if tc then
    if let Some(tc) = tc {
        // if Duel.IsEnvironment(CARD_NECROVALLEY) and tc:IsAbleToHand() and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
        if duel::is_environment(d, CARD_NECROVALLEY, None, None)
            && card::is_able_to_hand(d, tc, None)
            && duel::select_yes_no(d, tp, aux::stringid(ID, 0))
        {
            // Duel.SendtoHand(tc,nil,REASON_EFFECT)
            duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,tc)
            duel::confirm_cards(d, 1 - tp, CardSet::from_card(tc));
        } else {
            // Duel.ShuffleDeck(tp)
            duel::shuffle_deck(d, tp);
            // Duel.MoveSequence(tc,0)
            duel::move_sequence(d, tc, 0, None);
            // Duel.ConfirmDecktop(tp,1)
            duel::confirm_decktop(d, tp, 1);
        }
    }
}
