//! Naturia Rosewhip
use super::prelude::*;

pub const ID: u32 = 61049315;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate limit
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetOperation(s.aclimit1)
    effect::set_operation(d, e1, Some(operation(aclimit1)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e2:SetCode(EVENT_CHAIN_NEGATED)
    effect::set_code(d, e2, EVENT_CHAIN_NEGATED);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetOperation(s.aclimit2)
    effect::set_operation(d, e2, Some(operation(aclimit2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e3, EFFECT_CANNOT_ACTIVATE);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(0,1)
    effect::set_target_range(d, e3, 0, 1);
    // e3:SetCondition(s.econ)
    effect::set_condition(d, e3, Some(condition(econ)));
    // e3:SetValue(s.elimit)
    effect::set_value_fn(d, e3, lua_fn(elimit));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}

/// `s.aclimit1(e,tp,eg,ep,ev,re,r,rp)`
fn aclimit1(d: &mut Duel, a: &EffArgs) {
    // if ep==tp or not re:IsHasType(EFFECT_TYPE_ACTIVATE) then return end
    if a.ep == a.tp || !effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE) {
        return;
    }
    // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD_DISABLE|RESET_CONTROL|RESET_PHASE|PHASE_END,0,1)
    let c = effect::get_handler(d, a.e).nil();
    let _ = card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD_DISABLE | RESET_CONTROL | RESET_PHASE | PHASE_END, 0, 1, 0, 0);
}

/// `s.aclimit2(e,tp,eg,ep,ev,re,r,rp)`
fn aclimit2(d: &mut Duel, a: &EffArgs) {
    // if ep==tp or not re:IsHasType(EFFECT_TYPE_ACTIVATE) then return end
    if a.ep == a.tp || !effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE) {
        return;
    }
    // e:GetHandler():ResetFlagEffect(id)
    let c = effect::get_handler(d, a.e).nil();
    card::reset_flag_effect(d, c, ID);
}

/// `s.econ(e)`: `e:GetHandler():GetFlagEffect(id)~=0`
fn econ(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, c, ID) != 0
}

/// `s.elimit(e,te,tp)`: `te:IsHasType(EFFECT_TYPE_ACTIVATE)`
fn elimit(d: &mut Duel, a: Args) -> bool {
    effect::is_has_type(d, a.e(1), EFFECT_TYPE_ACTIVATE)
}
