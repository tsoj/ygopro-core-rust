//! Goblin Pothole Squad
use super::prelude::*;

pub const ID: u32 = 12755462;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TRAP_HOLE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // summon success
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetOperation(s.sumsuc)
    effect::set_operation(d, e1, Some(operation(sumsuc)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // activate limit
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetOperation(s.cedop)
    effect::set_operation(d, e2, Some(operation(cedop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SUMMON_SUCCESS);
    // e3:SetCondition(s.cedcon)
    effect::set_condition(d, e3, Some(condition(cedcon)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e2:Clone()
    let e4 = effect::clone(d, e2);
    // e4:SetCode(EVENT_FLIP_SUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_FLIP_SUMMON_SUCCESS);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetCode(EVENT_CHAIN_END)
    effect::set_code(d, e5, EVENT_CHAIN_END);
    // e5:SetOperation(s.cedop2)
    effect::set_operation(d, e5, Some(operation(cedop2)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
// function s.sumsuc(e,tp,eg,ep,ev,re,r,rp)
fn sumsuc(d: &mut Duel, _a: &EffArgs) {
    // Duel.SetChainLimitTillChainEnd(s.chlimit1)
    duel::set_chain_limit_till_chain_end(d, lua_fn(chlimit1));
}
// function s.chlimit1(re,rp,tp)
fn chlimit1(d: &mut Duel, a: Args) -> bool {
    // return not re:GetHandler():IsTrap() or not re:IsHasType(EFFECT_TYPE_ACTIVATE)
    let h = effect::get_handler(d, a.e(0)).nil();
    !card::is_trap(d, h) || !effect::is_has_type(d, a.e(0), EFFECT_TYPE_ACTIVATE)
}
// function s.cedcon(e,tp,eg,ep,ev,re,r,rp)
fn cedcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:GetFirst()~=e:GetHandler()
    let eg = a.eg.clone().nil();
    let first = group::get_first(d, &eg);
    let h = effect::get_handler(d, a.e).nil();
    first != Some(h)
}
// function s.cedop(e,tp,eg,ep,ev,re,r,rp)
fn cedop(d: &mut Duel, _a: &EffArgs) {
    // Duel.SetChainLimitTillChainEnd(s.chlimit2)
    duel::set_chain_limit_till_chain_end(d, lua_fn(chlimit2));
}
// function s.cedop2(e,tp,eg,ep,ev,re,r,rp)
fn cedop2(d: &mut Duel, _a: &EffArgs) {
    // if Duel.CheckEvent(EVENT_SPSUMMON_SUCCESS) then
    if duel::check_event(d, EVENT_SPSUMMON_SUCCESS).is_some() {
        // Duel.SetChainLimitTillChainEnd(s.chlimit2)
        duel::set_chain_limit_till_chain_end(d, lua_fn(chlimit2));
    }
}
// function s.chlimit2(re,rp,tp)
fn chlimit2(d: &mut Duel, a: Args) -> bool {
    // return not re:GetHandler():IsTrap() or not re:IsHasType(EFFECT_TYPE_ACTIVATE) or not re:GetHandler():IsSetCard(SET_TRAP_HOLE)
    let h = effect::get_handler(d, a.e(0)).nil();
    if !card::is_trap(d, h) {
        return true;
    }
    if !effect::is_has_type(d, a.e(0), EFFECT_TYPE_ACTIVATE) {
        return true;
    }
    let h = effect::get_handler(d, a.e(0)).nil();
    !card::is_set_card(d, h, &[SET_TRAP_HOLE as u16])
}
