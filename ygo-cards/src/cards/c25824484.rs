//! Sylvan Hermitree
use super::prelude::*;

pub const ID: u32 = 25824484;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Excavate the top card of your Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DRAW+CATEGORY_DECKDES)
    effect::set_category(d, e1, CATEGORY_DRAW + CATEGORY_DECKDES);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Sort up to 3 cards from the top of your Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCondition(s.sdcon)
    effect::set_condition(d, e2, Some(condition(sdcon)));
    // e2:SetOperation(s.sdop)
    effect::set_operation(d, e2, Some(operation(sdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDiscardDeck(tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck(d, a.tp, 1);
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
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
        // Duel.SendtoGrave(tc,REASON_EFFECT|REASON_EXCAVATE)
        duel::send_to_grave(d, CardSet::from_card(tc), REASON_EFFECT | REASON_EXCAVATE, None, None);
        // local ccount=Duel.GetOperatedGroup():FilterCount(Card.IsLocation,nil,LOCATION_GRAVE)
        let og = duel::get_operated_group(d);
        let ccount = group::filter_count(d, &og, &|d, c| card::is_location(d, c, LOCATION_GRAVE), NO_EX);
        // if ccount>0 then
        if ccount > 0 {
            // Duel.Draw(tp,1,REASON_EFFECT)
            duel::draw(d, a.tp, 1, REASON_EFFECT);
        }
    } else {
        // Duel.MoveToDeckBottom(tc)
        duel::move_to_deck_bottom(d, tc);
    }
}
/// `s.sdcon(e,tp,eg,ep,ev,re,r,rp)`
fn sdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousLocation(LOCATION_DECK) and c:IsReason(REASON_EXCAVATE)
    card::is_previous_location(d, c, LOCATION_DECK) && card::is_reason(d, c, REASON_EXCAVATE)
}
/// `s.sdop(e,tp,eg,ep,ev,re,r,rp)`
fn sdop(d: &mut Duel, a: &EffArgs) {
    // local ct=math.min(Duel.GetFieldGroupCount(tp,LOCATION_DECK,0),3)
    let ct = duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0).min(3);
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // local ac=ct==1 and ct or Duel.AnnounceNumberRange(tp,1,ct)
    let ac = if ct == 1 { 1 } else { duel::announce_number_range(d, a.tp, Some(1), Some(ct as u32), &[]).nil().0 as i64 };
    // Duel.SortDecktop(tp,tp,ac)
    duel::sort_decktop(d, a.tp, a.tp, ac as u16);
}
