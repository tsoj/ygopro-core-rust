//! Sky Striker Maneuver - Vector Blast
use super::prelude::*;

pub const ID: u32 = 21623008;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DECKDES+CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_DECKDES + CATEGORY_TODECK);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetSequence()<5
    card::get_sequence(d, c) < 5
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil)
    !duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDiscardDeck(tp,2) and Duel.IsPlayerCanDiscardDeck(1-tp,2) end
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck(d, a.tp, 2) && duel::is_player_can_discard_deck(d, 1 - a.tp, 2);
    }
    true
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetSequence()>=5
    card::get_sequence(d, c) >= 5
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // Duel.DiscardDeck(tp,2,REASON_EFFECT)
    duel::discard_deck(d, a.tp, 2, REASON_EFFECT);
    // Duel.DiscardDeck(1-tp,2,REASON_EFFECT)
    duel::discard_deck(d, 1 - a.tp, 2, REASON_EFFECT);
    // if Duel.IsExistingMatchingCard(Card.IsSpell,tp,LOCATION_GRAVE,0,3,nil)
    //     and Duel.IsExistingMatchingCard(s.filter,tp,0,LOCATION_MZONE,1,nil)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if duel::is_existing_matching_card(d, Some(&card::is_spell), a.tp, LOCATION_GRAVE, 0, 3, NO_EX)
        && duel::is_existing_matching_card(d, Some(&filter), a.tp, 0, LOCATION_MZONE, 1, NO_EX)
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 0))
    {
        // local g=Duel.GetMatchingGroup(s.filter,tp,0,LOCATION_MZONE,nil)
        let g = duel::get_matching_group(d, Some(&filter), a.tp, 0, LOCATION_MZONE, NO_EX);
        // Duel.SendtoDeck(g,nil,SEQ_DECKTOP,REASON_EFFECT)
        duel::send_to_deck(d, g, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
    }
}
