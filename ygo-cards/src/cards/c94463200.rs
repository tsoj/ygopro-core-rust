//! Battle-Scarred
use super::prelude::*;

pub const ID: u32 = 94463200;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ARCHFIEND], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddPersistentProcedure(c,0,aux.FaceupFilter(Card.IsSetCard,SET_ARCHFIEND),nil,nil,TIMING_STANDBY_PHASE)
    aux::add_persistent_procedure(
        d,
        c,
        Some(0),
        Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_ARCHFIEND as u16]))),
        None,
        None,
        Some(TIMING_STANDBY_PHASE),
        None,
        None,
        None,
        None,
        None,
        false,
    );
    // lpcost
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EVENT_PAY_LPCOST)
    effect::set_code(d, e2, EVENT_PAY_LPCOST);
    // e2:SetCondition(s.lpcon)
    effect::set_condition(d, e2, Some(condition(lpcon)));
    // e2:SetOperation(s.lpop)
    effect::set_operation(d, e2, Some(operation(lpop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destroy
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e3, EVENT_LEAVE_FIELD);
    // e3:SetCondition(s.descon)
    effect::set_condition(d, e3, Some(condition(descon)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e4:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e4, EVENT_LEAVE_FIELD);
    // e4:SetOperation(s.desop2)
    effect::set_operation(d, e4, Some(operation(desop2)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.lpcon(e,tp,eg,ep,ev,re,r,rp)`:
/// `return Duel.IsPhase(PHASE_STANDBY) and ep==tp and re:GetHandler()==e:GetHandler():GetFirstCardTarget()`
fn lpcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_phase(d, PHASE_STANDBY)
        && a.ep == a.tp
        && effect::get_handler(d, a.re.nil()) == card::get_first_card_target(d, effect::get_handler(d, a.e).nil())
}
/// `s.lpop(e,tp,eg,ep,ev,re,r,rp)`: `Duel.PayLPCost(1-ep,ev)`
fn lpop(d: &mut Duel, a: &EffArgs) {
    duel::pay_lp_cost(d, 1 - a.ep, a.ev as u32);
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsStatus(STATUS_DESTROY_CONFIRMED) then return false end
    if card::is_status(d, c, STATUS_DESTROY_CONFIRMED) {
        return false;
    }
    // local tc=c:GetFirstCardTarget()
    // return tc and eg:IsContains(tc) and tc:IsReason(REASON_DESTROY)
    let Some(tc) = card::get_first_card_target(d, c) else {
        return false;
    };
    let eg = a.eg.clone().nil();
    group::is_contains(&eg, tc) && card::is_reason(d, tc, REASON_DESTROY)
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`: `Duel.Destroy(e:GetHandler(), REASON_EFFECT)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    duel::destroy(d, h, REASON_EFFECT, None, None);
}
/// `s.desop2(e,tp,eg,ep,ev,re,r,rp)`
fn desop2(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetHandler():GetFirstCardTarget()
    let h = effect::get_handler(d, a.e).nil();
    // if tc and tc:IsLocation(LOCATION_MZONE) then Duel.Destroy(tc,REASON_EFFECT) end
    if let Some(tc) = card::get_first_card_target(d, h) {
        if card::is_location(d, tc, LOCATION_MZONE) {
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
