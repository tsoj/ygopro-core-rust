//! Digit Jamming
use super::prelude::*;

pub const ID: u32 = 49658464;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Change ATK/DEF
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e2, EVENT_PRE_DAMAGE_CALCULATE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation(true))
    effect::set_operation(d, e2, Some(operation(op_check)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Change ATK/DEF on destruction
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetOperation(s.operation(false))
    effect::set_operation(d, e3, Some(operation(op_destroy)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_attack_above(d, c, 1000)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    true
}
/// `s.operation(true)`
fn op_check(d: &mut Duel, a: &EffArgs) {
    operation_impl(d, a, true);
}
/// `s.operation(false)`
fn op_destroy(d: &mut Duel, a: &EffArgs) {
    operation_impl(d, a, false);
}
/// The function returned by `s.operation(check)`.
fn operation_impl(d: &mut Duel, a: &EffArgs, check: bool) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local reset=0
    let reset: u32;
    if check {
        // if not c:IsRelateToEffect(e) then return end
        if !card::is_relate_to_effect(d, c, a.e) {
            return;
        }
        // reset=RESET_EVENT|RESETS_STANDARD
        reset = RESET_EVENT | RESETS_STANDARD;
    } else {
        // reset=RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_END
        reset = RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_END;
    }
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // local atk=math.floor(tc:GetAttack()/1000)*1000
        let atk = (card::get_attack(d, tc) as i64).div_euclid(1000) * 1000;
        // local def=math.floor(tc:GetDefense()/1000)*1000
        let def = (card::get_defense(d, tc) as i64).div_euclid(1000) * 1000;
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(tc:GetAttack()-atk)
        let v1 = card::get_attack(d, tc) as i64 - atk;
        effect::set_value(d, e1, v1);
        // e1:SetReset(reset)
        effect::set_reset(d, e1, reset, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_SET_DEFENSE_FINAL)
        effect::set_code(d, e2, EFFECT_SET_DEFENSE_FINAL);
        // e2:SetValue(tc:GetDefense()-def)
        let v2 = card::get_defense(d, tc) as i64 - def;
        effect::set_value(d, e2, v2);
        // e2:SetReset(reset)
        effect::set_reset(d, e2, reset, 1);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
    }
}
