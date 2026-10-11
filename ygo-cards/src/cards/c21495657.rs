//! Zefraxi, Treasure of the Yang Zing
use super::prelude::*;

pub const ID: u32 = 21495657;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_YANG_ZING, SET_ZEFRA], listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum Summon procedure
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // You cannot Pendulum Summon monsters, except "Yang Zing" and "Zefra" monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CANNOT_NEGATE)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CANNOT_NEGATE, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c,sump,sumtype,sumpos,targetp) return sumtype&SUMMON_TYPE_PENDULUM==SUMMON_TYPE_PENDULUM and not c:IsSetCard({SET_YANG_ZING,SET_ZEFRA}) end)
    effect::set_target(d, e1, Some(lua_fn(pend_target)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Treat 1 "Yang Zing" or "Zefra" monster you control, except "Zefraxi, Treasure of the Yang Zing", as a Tuner this turn
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.tunercon)
    effect::set_condition(d, e2, Some(condition(tunercon)));
    // e2:SetTarget(s.tunertg)
    effect::set_target(d, e2, Some(cost_target(tunertg)));
    // e2:SetOperation(s.tunerop)
    effect::set_operation(d, e2, Some(operation(tunerop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function(e,c,sump,sumtype,sumpos,targetp) return sumtype&SUMMON_TYPE_PENDULUM==SUMMON_TYPE_PENDULUM and not c:IsSetCard({SET_YANG_ZING,SET_ZEFRA}) end
fn pend_target(d: &mut Duel, a: Args) -> bool {
    let sumtype = a.int(3) as u32;
    let c = a.c(1);
    // sumtype&SUMMON_TYPE_PENDULUM==SUMMON_TYPE_PENDULUM
    (sumtype & SUMMON_TYPE_PENDULUM) == SUMMON_TYPE_PENDULUM && !card::is_set_card(d, c, &[SET_YANG_ZING as u16, SET_ZEFRA as u16])
}
// function s.tunercon(e,tp,eg,ep,ev,re,r,rp)
fn tunercon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPendulumSummoned() or c:IsSummonLocation(LOCATION_DECK)
    card::is_pendulum_summoned(d, c) || card::is_summon_location(d, c, LOCATION_DECK)
}
// function s.tunerfilter(c)
fn tunerfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard({SET_YANG_ZING,SET_ZEFRA}) and not c:IsType(TYPE_TUNER) and c:IsFaceup() and not c:IsCode(id)
    card::is_set_card(d, c, &[SET_YANG_ZING as u16, SET_ZEFRA as u16])
        && !card::is_type(d, c, TYPE_TUNER)
        && card::is_faceup(d, c)
        && !card::is_code(d, c, &[ID])
}
// function s.tunertg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn tunertg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.tunerfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && tunerfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tunerfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tunerfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.tunerfilter,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, a.tp, Some(&tunerfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
// function s.tunerop(e,tp,eg,ep,ev,re,r,rp)
fn tunerop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // That face-up monster is treated as a Tuner this turn
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_ADD_TYPE)
        effect::set_code(d, e1, EFFECT_ADD_TYPE);
        // e1:SetValue(TYPE_TUNER)
        effect::set_value(d, e1, TYPE_TUNER as i64);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Place this card on the bottom of the Deck when it leaves the field
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetDescription(3301)
        effect::set_description(d, e2, 3301);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e2:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
        effect::set_code(d, e2, EFFECT_LEAVE_FIELD_REDIRECT);
        // e2:SetValue(LOCATION_DECKBOT)
        effect::set_value(d, e2, LOCATION_DECKBOT as i64);
        // e2:SetReset(RESET_EVENT|RESETS_REDIRECT)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_REDIRECT, 1);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
    }
}
