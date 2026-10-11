//! Miniaturize
//! ミニチュアライズ
use super::prelude::*;

pub const ID: u32 = 34815282;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddPersistentProcedure(c,nil,s.filter,CATEGORY_ATKCHANGE,EFFECT_FLAG_DAMAGE_STEP,TIMING_DAMAGE_STEP,TIMING_DAMAGE_STEP|TIMINGS_CHECK_MONSTER,aux.StatChangeDamageStepCondition)
    aux::add_persistent_procedure(
        d,
        c,
        None,
        Some(aux::equip_filter_fn(|d, c, _e, _tp| filter(d, c))),
        Some(CATEGORY_ATKCHANGE),
        Some(EFFECT_FLAG_DAMAGE_STEP),
        Some(TIMING_DAMAGE_STEP),
        Some(TIMING_DAMAGE_STEP | TIMINGS_CHECK_MONSTER),
        Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))),
        None,
        None,
        None,
        false,
    );
    // eff
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetTarget(aux.PersistentTargetFilter)
    effect::set_target(d, e1, Some(aux::persistent_target_filter()));
    // e1:SetValue(-1000)
    effect::set_value(d, e1, -1000);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_UPDATE_LEVEL)
    effect::set_code(d, e2, EFFECT_UPDATE_LEVEL);
    // e2:SetValue(-1)
    effect::set_value(d, e2, -1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destroy
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
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
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:GetBaseAttack()>1000 and c:HasLevel() and c:GetLevel()~=1
    card::is_faceup(d, c) && card::get_base_attack(d, c) > 1000 && card::has_level(d, c) && card::get_level(d, c) != 1
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
    let tc = card::get_first_card_target(d, c);
    // return tc and eg:IsContains(tc)
    match tc {
        Some(tc) => {
            let eg = a.eg.clone().nil();
            group::is_contains(&eg, tc)
        }
        None => false,
    }
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // Duel.Destroy(e:GetHandler(),REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    duel::destroy(d, h, REASON_EFFECT, None, None);
}
