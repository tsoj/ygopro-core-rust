//! Field-Commander Rahz
use super::prelude::*;

pub const ID: u32 = 89529919;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelBelow(4) and c:IsRace(RACE_WARRIOR) and c:GetCode()~=id
    card::is_level_below(d, c, 4) && card::is_race(d, c, RACE_WARRIOR) && card::get_code(d, c)[0] != ID
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 1));
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| filter(d, c)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    // if tc then
    if let Some(tc) = group::get_first(d, &g) {
        // Duel.ShuffleDeck(tp)
        duel::shuffle_deck(d, tp);
        // Duel.MoveSequence(tc,0)
        duel::move_sequence(d, tc, 0, None);
        // Duel.ConfirmDecktop(tp,1)
        duel::confirm_decktop(d, tp, 1);
    }
}
