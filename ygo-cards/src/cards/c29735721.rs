//! Spell Vanishing
use super::prelude::*;

pub const ID: u32 = 29735721;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY+CATEGORY_DECKDES+CATEGORY_HANDES)
    effect::set_category(d, e1, CATEGORY_NEGATE | CATEGORY_DESTROY | CATEGORY_DECKDES | CATEGORY_HANDES);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // return re:IsSpellEffect() and re:IsHasType(EFFECT_TYPE_ACTIVATE) and Duel.IsChainNegatable(ev)
    let re = a.re.nil();
    effect::is_spell_effect(d, re) && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.cost(e,tp,...,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if Duel.IsPlayerAffectedByEffect(tp,EFFECT_DISCARD_COST_CHANGE) then return true end
    if duel::is_player_affected_by_effect(d, a.tp, EFFECT_DISCARD_COST_CHANGE).is_some() {
        return true;
    }
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,2,e:GetHandler()) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, None, None)), a.tp, LOCATION_HAND, 0, 2, h.into());
    }
    // Duel.DiscardHand(tp,Card.IsDiscardable,2,2,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, a.tp, Some(&|d, c| card::is_discardable(d, c, None, None)), 2, 2, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,0,LOCATION_HAND|LOCATION_DECK)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND | LOCATION_DECK) > 0;
    }
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, a.re.nil()).nil();
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // if rc:IsDestructable() and rc:IsRelateToEffect(re) then
    if card::is_destructable(d, rc, None) && card::is_relate_to_effect(d, rc, a.re.nil()) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,1-tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, 1 - a.tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, a.re.nil()).nil();
    // if Duel.NegateActivation(ev) and rc:IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) && card::is_relate_to_effect(d, rc, a.re.nil()) {
        // Duel.Destroy(eg,REASON_EFFECT)
        duel::destroy(d, a.eg.clone().nil(), REASON_EFFECT, None, None);
    }
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_HAND|LOCATION_DECK)
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_HAND | LOCATION_DECK);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // Duel.BreakEffect()
    duel::break_effect(d);
    // Duel.ConfirmCards(tp,g)
    duel::confirm_cards(d, a.tp, g.clone());
    // local sg=g:Filter(Card.IsCode,nil,rc:GetCode())
    let code = card::get_code(d, rc)[0];
    let sg = group::filter(d, &g, &|d, c| card::is_code(d, c, &[code]), NO_EX);
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.SendtoGrave(sg,REASON_EFFECT)
        duel::send_to_grave(d, sg, REASON_EFFECT, None, None);
    }
    // Duel.ShuffleHand(1-tp)
    duel::shuffle_hand(d, 1 - a.tp);
    // Duel.ShuffleDeck(1-tp)
    duel::shuffle_deck(d, 1 - a.tp);
}
