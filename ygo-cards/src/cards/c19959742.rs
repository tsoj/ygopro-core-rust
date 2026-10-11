//! Gishki Mollusk
//! リチュア・シェルフィッシュ
use super::prelude::*;

pub const ID: u32 = 19959742;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // confirm
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return (r&REASON_EFFECT)~=0 and Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>=3
    (a.r & REASON_EFFECT) != 0 && duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) >= 3
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)<3 then return end
    if duel::get_field_group_count(d, tp, LOCATION_DECK, 0) < 3 {
        return;
    }
    // Duel.SortDecktop(tp,tp,3)
    duel::sort_decktop(d, tp, tp, 3);
    // if Duel.SelectOption(tp,aux.Stringid(id,1),aux.Stringid(id,2))==1 then
    if duel::select_option(d, tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2)]) == 1 {
        // Duel.MoveToDeckBottom(3,tp)
        duel::move_decktop_to_bottom(d, 3, tp);
    }
}
