//! ペンデュラム・モラトリアム
//! Pendulum Impenetrable
use super::prelude::*;

pub const ID: u32 = 60434189;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    // e1:SetTargetRange(LOCATION_PZONE,LOCATION_PZONE)
    effect::set_target_range(d, e1, LOCATION_PZONE, LOCATION_PZONE);
    // e1:SetValue(s.indval)
    effect::set_value_fn(d, e1, lua_fn(indval));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e2, EVENT_CHAIN_SOLVING);
    // e2:SetCondition(s.discon)
    effect::set_condition(d, e2, Some(condition(discon)));
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
}
fn indval(d: &mut Duel, a: Args) -> bool {
    // return tp~=e:GetHandlerPlayer()
    let e = a.e(0);
    let tp = a.p(2);
    tp != effect::get_handler_player(d, e)
}
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // if not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return false end
    let Some(re) = a.re else { return false };
    if !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return false;
    }
    // local g=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]);
    let g = info[0].as_group().cloned();
    // return g and g:IsExists(s.indfilter,1,nil) and ep~=tp
    match g {
        Some(g) => group::is_exists(d, &g, &indfilter, 1, NO_EX) && a.ep != a.tp,
        None => false,
    }
}
fn indfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLocation(LOCATION_PZONE)
    card::is_location(d, c, LOCATION_PZONE)
}
fn disop(d: &mut Duel, a: &EffArgs) {
    // Duel.NegateEffect(ev)
    duel::negate_effect(d, a.ev as u8);
}
