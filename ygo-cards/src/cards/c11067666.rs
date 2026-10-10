//! White Wing Magician
use super::prelude::*;

pub const ID: u32 = 11067666;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Negate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e1, EVENT_CHAIN_SOLVING);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Banish
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
    effect::set_code(d, e2, EFFECT_LEAVE_FIELD_REDIRECT);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetValue(LOCATION_REMOVED)
    effect::set_value(d, e2, LOCATION_REMOVED as i64);
    // e2:SetCondition(s.rmcon)
    effect::set_condition(d, e2, Some(condition(rmcon)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// s.cfilter(c,tp)
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsLocation(LOCATION_MZONE) and c:IsFaceup() and c:IsRace(RACE_SPELLCASTER) and c:IsAttribute(ATTRIBUTE_DARK) and c:IsControler(tp)
    card::is_location(d, c, LOCATION_MZONE)
        && card::is_faceup(d, c)
        && card::is_race(d, c, RACE_SPELLCASTER)
        && card::is_attribute(d, c, ATTRIBUTE_DARK)
        && card::is_controler(d, c, tp)
}
// s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]);
    let g = info[0].as_group().cloned();
    // return e:GetHandler():GetFlagEffect(id)==0 and re:IsHasProperty(EFFECT_FLAG_CARD_TARGET)
    //     and g and g:IsExists(s.cfilter,1,nil,tp) and Duel.IsChainDisablable(ev)
    let c = effect::get_handler(d, a.e).nil();
    if card::get_flag_effect(d, c, ID) != 0 {
        return false;
    }
    let re = a.re.nil();
    if !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return false;
    }
    match g {
        Some(g) => group::is_exists(d, &g, &|d, x| cfilter(d, x, a.tp), 1, NO_EX) && duel::is_chain_disablable(d, a.ev as u8),
        None => false,
    }
}
// s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // if Duel.SelectEffectYesNo(tp,e:GetHandler()) then
    let c = effect::get_handler(d, a.e).nil();
    if duel::select_effect_yes_no(d, a.tp, c, None) {
        // e:GetHandler():RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
        card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
        // if Duel.NegateEffect(ev) then
        if duel::negate_effect(d, a.ev as u8) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Destroy(e:GetHandler(),REASON_EFFECT)
            duel::destroy(d, c, REASON_EFFECT, None, None);
        }
    }
}
// s.rmcon(e)
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPendulumSummoned() and c:IsReason(REASON_MATERIAL) and c:IsReason(REASON_SYNCHRO)
    card::is_pendulum_summoned(d, c) && card::is_reason(d, c, REASON_MATERIAL) && card::is_reason(d, c, REASON_SYNCHRO)
}
