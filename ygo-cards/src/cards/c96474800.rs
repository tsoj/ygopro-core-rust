//! Magic Deflector
//! (マジック・ディフレクター)
use super::prelude::*;

pub const ID: u32 = 96474800;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
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
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // disable
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_DISABLE)
    effect::set_code(d, e1, EFFECT_DISABLE);
    // e1:SetTargetRange(LOCATION_SZONE,LOCATION_SZONE)
    effect::set_target_range(d, e1, LOCATION_SZONE, LOCATION_SZONE);
    // e1:SetTarget(s.distg)
    effect::set_target(d, e1, Some(effect_card_fn(distg)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // disable effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e2, EVENT_CHAIN_SOLVING);
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
}
/// `s.distg(e,c)`
fn distg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // local tpe=c:GetType()
    let tpe = card::get_type(d, c);
    // return (tpe&TYPE_SPELL)~=0 and (tpe&TYPE_EQUIP+TYPE_FIELD+TYPE_CONTINUOUS+TYPE_QUICKPLAY)~=0
    (tpe & TYPE_SPELL) != 0 && (tpe & (TYPE_EQUIP + TYPE_FIELD + TYPE_CONTINUOUS + TYPE_QUICKPLAY)) != 0
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // local tl=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION)
    let tl = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION])[0].int();
    // local tpe=re:GetActiveType()
    let re = a.re.nil();
    let tpe = effect::get_active_type(d, re);
    // if (tl&LOCATION_SZONE)~=0 and (tpe&TYPE_SPELL)~=0 and (tpe&TYPE_EQUIP+TYPE_FIELD+TYPE_CONTINUOUS+TYPE_QUICKPLAY)~=0 then
    if (tl & LOCATION_SZONE as i64) != 0 && (tpe & TYPE_SPELL) != 0 && (tpe & (TYPE_EQUIP + TYPE_FIELD + TYPE_CONTINUOUS + TYPE_QUICKPLAY)) != 0 {
        // Duel.NegateEffect(ev)
        duel::negate_effect(d, a.ev as u8);
    }
}
