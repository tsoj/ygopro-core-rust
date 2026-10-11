//! Jade Insect Whistle
use super::prelude::*;

pub const ID: u32 = 95214051;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
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
    // if chk==0 then return Duel.GetFieldGroupCount(tp,0,LOCATION_DECK)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, 0, LOCATION_DECK) > 0;
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,1-tp,aux.Stringid(id,0))
    duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, aux::stringid(ID, 0));
    // local g=Duel.SelectMatchingCard(1-tp,Card.IsRace,1-tp,LOCATION_DECK,0,1,1,nil,RACE_INSECT)
    let g = duel::select_matching_card(d, 1 - tp, Some(&|d, c| card::is_race(d, c, RACE_INSECT as u64)), 1 - tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // if tc then
    if let Some(tc) = tc {
        // Duel.ShuffleDeck(1-tp)
        duel::shuffle_deck(d, 1 - tp);
        // Duel.MoveSequence(tc,0)
        duel::move_sequence(d, tc, 0, None);
        // Duel.ConfirmDecktop(1-tp,1)
        let _ = duel::confirm_decktop(d, 1 - tp, 1);
    }
}
