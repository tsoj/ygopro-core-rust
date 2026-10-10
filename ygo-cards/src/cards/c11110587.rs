//! That Grass Looks Greener
use super::prelude::*;

pub const ID: u32 = 11110587;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DECKDES)
    effect::set_category(d, e1, CATEGORY_DECKDES);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,...)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>Duel.GetFieldGroupCount(tp,0,LOCATION_DECK)
    duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) > duel::get_field_group_count(d, a.tp, 0, LOCATION_DECK)
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local ct=Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)-Duel.GetFieldGroupCount(tp,0,LOCATION_DECK)
    let ct = duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) - duel::get_field_group_count(d, a.tp, 0, LOCATION_DECK);
    // if chk==0 then return ct>0 and Duel.IsPlayerCanDiscardDeck(tp,ct) end
    if a.chk == Some(0) {
        return ct > 0 && duel::is_player_can_discard_deck(d, a.tp, ct as u32);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DECKDES,nil,0,tp,ct)
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, a.tp, ct);
    true
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local ct=Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)-Duel.GetFieldGroupCount(tp,0,LOCATION_DECK)
    let ct = duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) - duel::get_field_group_count(d, a.tp, 0, LOCATION_DECK);
    // if ct>0 then
    if ct > 0 {
        // Duel.DiscardDeck(tp,ct,REASON_EFFECT)
        duel::discard_deck(d, a.tp, ct as u16, REASON_EFFECT);
    }
}
