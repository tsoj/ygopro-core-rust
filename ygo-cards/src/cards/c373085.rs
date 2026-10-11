//! Firebrand Hymnist
use super::prelude::*;

pub const ID: u32 = 373085;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --deck destruction
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DECKDES)
    effect::set_category(d, e1, CATEGORY_DECKDES);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DECKDES)
    effect::set_category(d, e2, CATEGORY_DECKDES);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.target2)
    effect::set_target(d, e2, Some(cost_target(target2)));
    // e2:SetOperation(s.operation2)
    effect::set_operation(d, e2, Some(operation(operation2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)
    let g = duel::get_field_group_count(d, a.tp, 0, LOCATION_ONFIELD);
    // if chk==0 then return g>0 and Duel.IsPlayerCanDiscardDeck(1-tp,g) end
    if a.chk == Some(0) {
        return g > 0 && duel::is_player_can_discard_deck(d, 1 - a.tp, g as u32);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DECKDES,nil,0,1-tp,g)
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, 1 - a.tp, g);
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // Duel.DiscardDeck(1-tp,Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD),REASON_EFFECT)
    let count = duel::get_field_group_count(d, a.tp, 0, LOCATION_ONFIELD);
    duel::discard_deck(d, 1 - a.tp, count as u16, REASON_EFFECT);
}
// function s.target2(e,tp,eg,ep,ev,re,r,rp,chk)
fn target2(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)
    let g = duel::get_field_group_count(d, a.tp, 0, LOCATION_ONFIELD);
    // if chk==0 then return g>0 and Duel.IsPlayerCanDiscardDeck(tp,g) end
    if a.chk == Some(0) {
        return g > 0 && duel::is_player_can_discard_deck(d, a.tp, g as u32);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DECKDES,nil,0,tp,g)
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, a.tp, g);
    true
}
// function s.operation2(e,tp,eg,ep,ev,re,r,rp)
fn operation2(d: &mut Duel, a: &EffArgs) {
    // Duel.DiscardDeck(tp,Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD),REASON_EFFECT)
    let count = duel::get_field_group_count(d, a.tp, 0, LOCATION_ONFIELD);
    duel::discard_deck(d, a.tp, count as u16, REASON_EFFECT);
}
