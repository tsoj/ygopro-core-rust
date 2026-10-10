//! Dice Re-Roll
use super::prelude::*;

pub const ID: u32 = 83241722;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetOperation(s.regop)
    effect::set_operation(d, e1, Some(operation(regop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.regop(e,tp,...)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_TOSS_DICE_NEGATE)
    effect::set_code(d, e1, EVENT_TOSS_DICE_NEGATE);
    // e1:SetCondition(s.coincon)
    effect::set_condition(d, e1, Some(condition(coincon)));
    // e1:SetOperation(s.coinop)
    effect::set_operation(d, e1, Some(operation(coinop)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `s.coincon(e,tp,...)`
fn coincon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFlagEffect(tp,id)==0
    duel::get_flag_effect(d, a.tp, ID) == 0
}
/// `s.coinop(e,tp,eg,ep,ev,re,r,rp)`
fn coinop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetFlagEffect(tp,id)~=0 then return end
    if duel::get_flag_effect(d, a.tp, ID) != 0 {
        return;
    }
    // if Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if duel::select_yes_no(d, a.tp, aux::stringid(ID, 0)) {
        // Duel.Hint(HINT_CARD,0,id)
        duel::hint(d, HINT_CARD as u8, 0, ID as u64);
        // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        // local ct1=(ev&0xff)
        let ct1 = a.ev & 0xff;
        // local ct2=(ev>>16)
        let ct2 = a.ev >> 16;
        // Duel.TossDice(ep,ct1,ct2)
        duel::toss_dice(d, a.ep, ct1 as u8, ct2 as u8);
    }
}
