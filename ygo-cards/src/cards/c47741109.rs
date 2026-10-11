//! Sylvan Snapdrassinagon
use super::prelude::*;

pub const ID: u32 = 47741109;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Excavate the top card of your Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DECKDES)
    effect::set_category(d, e1, CATEGORY_DECKDES);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCondition(s.condition2)
    effect::set_condition(d, e2, Some(condition(cond2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousLocation(LOCATION_HAND|LOCATION_ONFIELD)
    card::is_previous_location(d, c, LOCATION_HAND | LOCATION_ONFIELD)
}
// function s.condition2(e,tp,eg,ep,ev,re,r,rp)
fn cond2(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousLocation(LOCATION_DECK) and c:IsReason(REASON_EXCAVATE)
    card::is_previous_location(d, c, LOCATION_DECK) && card::is_reason(d, c, REASON_EXCAVATE)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDiscardDeck(tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck(d, a.tp, 1);
    }
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // if not Duel.IsPlayerCanDiscardDeck(tp,1) then return end
    if !duel::is_player_can_discard_deck(d, a.tp, 1) {
        return;
    }
    // Duel.ConfirmDecktop(tp,1)
    duel::confirm_decktop(d, a.tp, 1);
    // local g=Duel.GetDecktopGroup(tp,1)
    let g = duel::get_decktop_group(d, a.tp, 1);
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g).nil();
    // if tc:IsRace(RACE_PLANT) then
    if card::is_race(d, tc, RACE_PLANT) {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // Duel.SendtoGrave(g,REASON_EFFECT|REASON_EXCAVATE)
        duel::send_to_grave(d, g, REASON_EFFECT | REASON_EXCAVATE, None, None);
    } else {
        // Duel.MoveSequence(tc,1)
        duel::move_sequence(d, tc, 1, None);
    }
}
