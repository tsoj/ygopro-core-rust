//! Flourishing Frolic
use super::prelude::*;

pub const ID: u32 = 32887445;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_TOGRAVE);
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
    // if chk==0 then
    if a.chk == Some(0) {
        // local ct=Duel.GetMatchingGroupCount(Card.IsCode,tp,LOCATION_GRAVE,0,nil,id)+3
        let ct = duel::get_matching_group_count(d, Some(&|d, c| card::is_code(d, c, &[ID])), a.tp, LOCATION_GRAVE, 0, NO_EX) + 3;
        // if Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)<ct then return false end
        if (duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) as usize) < ct {
            return false;
        }
        // local g=Duel.GetDecktopGroup(tp,ct)
        let g = duel::get_decktop_group(d, a.tp, ct as u32);
        // local result=g:FilterCount(Card.IsAbleToHand,nil)>0
        let result = group::filter_count(d, &g, &|d, c| card::is_able_to_hand(d, c, None), NO_EX) > 0;
        // return result and Duel.IsPlayerCanDiscardDeck(tp,ct)
        return result && duel::is_player_can_discard_deck(d, a.tp, ct as u32);
    }
    true
}
// function s.filter(c)
fn filter_type(d: &mut Duel, c: CardId) -> i64 {
    // return c:GetType()&(TYPE_MONSTER|TYPE_SPELL|TYPE_TRAP)
    (card::get_type(d, c) & (TYPE_MONSTER | TYPE_SPELL | TYPE_TRAP)) as i64
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local ct=Duel.GetMatchingGroupCount(Card.IsCode,tp,LOCATION_GRAVE,0,nil,id)+3
    let ct = duel::get_matching_group_count(d, Some(&|d, c| card::is_code(d, c, &[ID])), a.tp, LOCATION_GRAVE, 0, NO_EX) as u32 + 3;
    // Duel.ConfirmDecktop(tp,ct)
    duel::confirm_decktop(d, a.tp, ct);
    // local g=Duel.GetDecktopGroup(tp,ct)
    let mut g = duel::get_decktop_group(d, a.tp, ct);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // if g:GetClassCount(s.filter)==3 then
    if group::get_class_count(d, &g, &filter_type) == 3 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local sg=g:FilterSelect(tp,Card.IsAbleToHand,1,1,nil)
        let sg = group::filter_select(d, &g, a.tp, &|d, c| card::is_able_to_hand(d, c, None), 1, 1, false, NO_EX).nil();
        // if #sg>0 then
        if !sg.is_empty() {
            // Duel.SendtoHand(sg,nil,REASON_EFFECT)
            duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,sg)
            duel::confirm_cards(d, 1 - a.tp, sg.clone());
            // g:RemoveCard(sg:GetFirst())
            let first = group::get_first(d, &sg).nil();
            group::remove_card(&mut g, &CardSet::from_card(first));
        }
        // Duel.SendtoGrave(g,REASON_EFFECT|REASON_EXCAVATE)
        duel::send_to_grave(d, g, REASON_EFFECT | REASON_EXCAVATE, None, None);
    } else {
        // Duel.ShuffleDeck(tp)
        duel::shuffle_deck(d, a.tp);
    }
}
