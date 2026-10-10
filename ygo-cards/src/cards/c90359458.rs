//! Top Share
use super::prelude::*;

pub const ID: u32 = 90359458;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Place on top
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.pltg)
    effect::set_target(d, e1, Some(cost_target(pltg)));
    // e1:SetOperation(s.plop)
    effect::set_operation(d, e1, Some(operation(plop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.pltg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn pltg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // return Duel.IsExistingMatchingCard(aux.NOT(Card.IsPublic),tp,LOCATION_DECK,0,1,nil)
        //     and Duel.IsExistingMatchingCard(aux.NOT(Card.IsPublic),tp,0,LOCATION_DECK,1,nil)
        return duel::is_existing_matching_card(d, Some(&|d, c| !card::is_public(d, c)), a.tp, LOCATION_DECK, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| !card::is_public(d, c)), a.tp, 0, LOCATION_DECK, 1, NO_EX);
    }
    true
}
/// `s.plop(e,tp,eg,ep,ev,re,r,rp)`
fn plop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 1));
    // local tc=Duel.SelectMatchingCard(tp,aux.NOT(Card.IsPublic),tp,LOCATION_DECK,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, c| !card::is_public(d, c)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let Some(tc) = group::get_first(d, &sel) else { return };
    // Duel.ShuffleDeck(tp)
    duel::shuffle_deck(d, tp);
    // Duel.MoveSequence(tc,0)
    duel::move_sequence(d, tc, 0, None);
    // Duel.ConfirmDecktop(tp,1)
    duel::confirm_decktop(d, tp, 1);
    // Duel.Hint(HINT_SELECTMSG,1-tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, aux::stringid(ID, 1));
    // tc=Duel.SelectMatchingCard(1-tp,aux.NOT(Card.IsPublic),1-tp,LOCATION_DECK,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, 1 - tp, Some(&|d, c| !card::is_public(d, c)), 1 - tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let Some(tc) = group::get_first(d, &sel) else { return };
    // Duel.BreakEffect()
    duel::break_effect(d);
    // Duel.ShuffleDeck(1-tp)
    duel::shuffle_deck(d, 1 - tp);
    // Duel.MoveSequence(tc,0)
    duel::move_sequence(d, tc, 0, None);
    // Duel.ConfirmDecktop(1-tp,1)
    duel::confirm_decktop(d, 1 - tp, 1);
}
