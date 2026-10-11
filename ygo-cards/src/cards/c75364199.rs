//! Mutually Affured Destruction
use super::prelude::*;

pub const ID: u32 = 75364199;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    // e1:SetCode(EVENT_FREE_CHAIN)
    // e1:SetOperation(s.activate)
    // c:RegisterEffect(e1)
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(c)
    // e1:SetType(EFFECT_TYPE_FIELD)
    // e1:SetCode(EFFECT_PUBLIC)
    // e1:SetTargetRange(LOCATION_HAND,LOCATION_HAND)
    // e1:SetReset(RESET_PHASE|PHASE_END)
    // Duel.RegisterEffect(e1,tp)
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_PUBLIC);
    effect::set_target_range(d, e1, LOCATION_HAND, LOCATION_HAND);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, tp);
    // local e2=Effect.CreateEffect(c)
    // e2:SetDescription(aux.Stringid(id,0))
    // e2:SetType(EFFECT_TYPE_FIELD)
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    // e2:SetCode(EFFECT_CANNOT_ACTIVATE)
    // e2:SetTargetRange(1,1)
    // e2:SetValue(s.aclimit)
    // e2:SetReset(RESET_PHASE|PHASE_END)
    // Duel.RegisterEffect(e2,tp)
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_code(d, e2, EFFECT_CANNOT_ACTIVATE);
    effect::set_target_range(d, e2, 1, 1);
    effect::set_value_fn(d, e2, lua_fn(aclimit));
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e2, tp);
}
// function s.aclimit(e,re,tp)
fn aclimit(d: &mut Duel, a: Args) -> bool {
    // local rc=re:GetHandler()
    let re = a.e(1);
    let rc = effect::get_handler(d, re).nil();
    // return Duel.IsExistingMatchingCard(Card.IsCode,tp,0,LOCATION_HAND,1,nil,rc:GetCode())
    let codes = card::get_code(d, rc);
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_code(d, c, &codes)), a.p(2), 0, LOCATION_HAND, 1, NO_EX)
}
