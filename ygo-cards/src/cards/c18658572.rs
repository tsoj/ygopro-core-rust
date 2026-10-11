//! Goblin Circus
use super::prelude::*;

pub const ID: u32 = 18658572;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.cfcon)
    effect::set_condition(d, e2, Some(condition(cfcon)));
    // e2:SetOperation(s.cfop)
    effect::set_operation(d, e2, Some(operation(cfop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.cfcon(e,tp,eg,ep,ev,re,r,rp)
fn cfcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp) and Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)~=0
    duel::is_turn_player(d, a.tp) && duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) != 0
}
// function s.cfop(e,tp,eg,ep,ev,re,r,rp)
fn cfop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local g=Duel.GetFieldGroup(tp,LOCATION_DECK,0)
    let g = duel::get_field_group(d, tp, LOCATION_DECK, 0);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local tc=g:GetMinGroup(Card.GetSequence):GetFirst()
    let mg = group::get_min_group(d, &g, &|d, c| card::get_sequence(d, c) as i64).nil().0;
    let tc = group::get_first(d, &mg).nil();
    // Duel.MoveSequence(tc,0)
    duel::move_sequence(d, tc, 0, None);
    // Duel.ConfirmDecktop(tp,1)
    let _ = duel::confirm_decktop(d, tp, 1);
    // if tc:IsAbleToRemove(tp,POS_FACEDOWN) and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if card::is_able_to_remove(d, tc, Some(tp), Some(POS_FACEDOWN as u8), None)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
    {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // Duel.Remove(tc,POS_FACEDOWN,REASON_EFFECT)
        duel::remove(d, tc, POS_FACEDOWN as u8, REASON_EFFECT, None, None);
    }
}
