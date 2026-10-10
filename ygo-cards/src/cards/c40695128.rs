//! Maharaghi
use super::prelude::*;

pub const ID: u32 = 40695128;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Spirit.AddProcedure(c,EVENT_SUMMON_SUCCESS,EVENT_FLIP)
    spirit::add_procedure(d, c, &[EVENT_SUMMON_SUCCESS, EVENT_FLIP]);
    // Cannot be Special Summoned
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    card::register_effect(d, c, e1, false);
    // Look at top card next Draw Phase
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    effect::set_operation(d, e2, Some(operation(regop)));
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EVENT_FLIP);
    card::register_effect(d, c, e3, false);
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetFlagEffect(tp,id)~=0 then return end
    if duel::get_flag_effect(d, a.tp, ID) != 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_PREDRAW)
    effect::set_code(d, e1, EVENT_PREDRAW);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // e1:SetReset(RESET_PHASE|PHASE_DRAW|RESET_SELF_TURN,1)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_DRAW | RESET_SELF_TURN, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,2)
    duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 2, 0);
    // aux.RegisterClientHint(c,0,tp,1,0,aux.Stringid(id,1),RESET_PHASE|PHASE_DRAW|RESET_SELF_TURN,1)
    aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 1)), RESET_PHASE | PHASE_DRAW | RESET_SELF_TURN, Some(1));
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp) and Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>0
    duel::is_turn_player(d, a.tp) && duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) > 0
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetDecktopGroup(tp,1)
    let g = duel::get_decktop_group(d, a.tp, 1);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // Duel.ConfirmCards(tp,g)
    duel::confirm_cards(d, a.tp, g.clone());
    // local opt=Duel.SelectOption(tp,aux.Stringid(id,2),aux.Stringid(id,3))
    let opt = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 2), aux::stringid(ID, 3)]);
    // if opt==1 then
    if opt == 1 {
        // Duel.MoveSequence(g:GetFirst(),1)
        let first = group::get_first(d, &g).nil();
        duel::move_sequence(d, first, 1, None);
    }
}
