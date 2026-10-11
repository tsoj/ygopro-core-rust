//! Inzektor Axe - Zektahawk
use super::prelude::*;

pub const ID: u32 = 87973893;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_INZEKTOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,aux.FilterBoolFunction(Card.IsSetCard,SET_INZEKTOR))
    aux::add_equip_procedure(
        d,
        c,
        None,
        Some(aux::equip_filter_fn(|d, x, _e, _tp| card::is_set_card(d, x, &[SET_INZEKTOR as u16]))),
        None,
        None,
        None,
        None,
        None,
        None,
    );
    // Atk
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetValue(1000)
    effect::set_value(d, e2, 1000);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // actlimit
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e4, EVENT_ATTACK_ANNOUNCE);
    // e4:SetCondition(s.accon)
    effect::set_condition(d, e4, Some(condition(accon)));
    // e4:SetOperation(s.acop)
    effect::set_operation(d, e4, Some(operation(acop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.accon(e,tp,eg,ep,ev,re,r,rp)`
fn accon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:GetFirst()==e:GetHandler():GetEquipTarget()
    let first = group::get_first(d, &a.eg());
    let c = effect::get_handler(d, a.e).nil();
    first == card::get_equip_target(d, c)
}
/// `s.acop(e,tp,eg,ep,ev,re,r,rp)`
fn acop(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    // e1:SetTargetRange(1,1)
    effect::set_target_range(d, e1, 1, 1);
    // e1:SetValue(s.aclimit)
    effect::set_value_fn(d, e1, lua_fn(|d, a| aclimit(d, a.e(1))));
    // e1:SetReset(RESET_PHASE|PHASE_DAMAGE)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `s.aclimit(e,re,tp)`
fn aclimit(d: &mut Duel, re: EffectId) -> bool {
    // return re:IsHasType(EFFECT_TYPE_ACTIVATE)
    effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
}
