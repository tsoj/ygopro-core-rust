//! Pikeru's Second Sight
use super::prelude::*;

pub const ID: u32 = 58015506;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Activate
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
// s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_DRAW)
    effect::set_code(d, e1, EVENT_DRAW);
    // e1:SetOperation(s.cfop)
    effect::set_operation(d, e1, Some(operation(cfop)));
    // e1:SetReset(RESET_PHASE|PHASE_END|RESET_OPPO_TURN,2)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 2);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
// s.cfop(e,tp,eg,ep,ev,re,r,rp)
fn cfop(d: &mut Duel, a: &EffArgs) {
    // if ep==e:GetOwnerPlayer() then return end
    if a.ep == effect::get_owner_player(d, a.e) {
        return;
    }
    // Duel.ConfirmCards(1-ep,eg)
    duel::confirm_cards(d, 1 - a.ep, a.eg());
}
