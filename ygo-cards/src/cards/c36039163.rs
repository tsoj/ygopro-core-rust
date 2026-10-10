//! Penguin Knight
use super::prelude::*;

pub const ID: u32 = 36039163;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // to deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_TODECK);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    // e1:SetCondition(s.tdcon)
    effect::set_condition(d, e1, Some(condition(tdcon)));
    // e1:SetTarget(s.tdtg)
    effect::set_target(d, e1, Some(cost_target(tdtg)));
    // e1:SetOperation(s.tdop)
    effect::set_operation(d, e1, Some(operation(tdop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.tdcon(e,tp,eg,ep,ev,re,r,rp)`
fn tdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsPreviousLocation(LOCATION_DECK) and (r&REASON_EFFECT)~=0 and rp==1-tp
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, h, LOCATION_DECK) && (a.r & REASON_EFFECT) != 0 && a.rp == 1 - a.tp
}
/// `s.tdtg(e,tp,...,chk)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetFieldGroup(tp,LOCATION_GRAVE,0)
    let g = duel::get_field_group(d, a.tp, LOCATION_GRAVE, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, 0, 0);
    true
}
/// `s.tdop(e,tp,...)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetFieldGroup(tp,LOCATION_GRAVE,0)
    let g = duel::get_field_group(d, a.tp, LOCATION_GRAVE, 0);
    // if Duel.SendtoDeck(g,nil,SEQ_DECKTOP,REASON_EFFECT)~=0 then
    if duel::send_to_deck(d, g.clone(), None, SEQ_DECKTOP as i32, REASON_EFFECT, None) != 0 {
        // if g:IsExists(Card.IsLocation,1,nil,LOCATION_DECK) then Duel.ShuffleDeck(tp) end
        if group::is_exists(d, &g, &|d, c| card::is_location(d, c, LOCATION_DECK), 1, NO_EX) {
            duel::shuffle_deck(d, a.tp);
        }
    }
}
