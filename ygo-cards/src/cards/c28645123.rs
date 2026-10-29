//! A.I. Challenge You
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 28645123;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_IGNISTER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // e0:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e0, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e0:SetHintTiming(TIMING_DAMAGE_STEP,TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e0, TIMING_DAMAGE_STEP as u32, Some(TIMING_DAMAGE_STEP as u32));
    // e0:SetCondition(aux.StatChangeDamageStepCondition)
    effect::set_condition(d, e0, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Monsters your opponent controls lose 100 ATK for each card you control
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    // e1:SetValue(function(e,c) return Duel.GetFieldGroupCount(e:GetHandlerPlayer(),LOCATION_ONFIELD,0)*-100 end)
    effect::set_value_fn(d, e1, effect_value(atk_value));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If your "@Ignister" monster battles, your opponent cannot activate cards or effects until the end of the Damage Step
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e2, EFFECT_CANNOT_ACTIVATE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(0,1)
    effect::set_target_range(d, e2, 0, 1);
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // e2:SetCondition(s.actcon)
    effect::set_condition(d, e2, Some(condition(actcon)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // When your "@Ignister" monster is destroyed by battle: You can target 1 other Cyberse monster with 2300 ATK in your GY; Special Summon it
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(condition(spcon)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.atkval` (e1): `Duel.GetFieldGroupCount(e:GetHandlerPlayer(),LOCATION_ONFIELD,0)*-100`
fn atk_value(d: &mut Duel, e: EffectId) -> i64 {
    let p = effect::get_handler_player(d, e);
    duel::get_field_group_count(d, p, LOCATION_ONFIELD, 0) as i64 * -100
}
/// `s.acfilter(c,tp)`
fn acfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_IGNISTER) and c:IsControler(tp)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_IGNISTER as u16]) && card::is_controler(d, c, tp)
}
/// `s.actcon(e)`
fn actcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e);
    // local a=Duel.GetAttacker()
    let att = duel::get_attacker(d);
    // local d=Duel.GetAttackTarget()
    let dt = duel::get_attack_target(d);
    // return (a and s.acfilter(a,tp)) or (d and s.acfilter(d,tp))
    if let Some(att) = att {
        if acfilter(d, att, tp) {
            return true;
        }
    }
    if let Some(dt) = dt {
        return acfilter(d, dt, tp);
    }
    false
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_IGNISTER) and c:IsReason(REASON_BATTLE)
    //     and c:IsPreviousLocation(LOCATION_MZONE) and c:IsPreviousControler(tp)
    card::is_set_card(d, c, &[SET_IGNISTER as u16])
        && card::is_reason(d, c, REASON_BATTLE)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_controler(d, c, tp)
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, tp), 1, NO_EX)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_CYBERSE) and c:IsAttack(2300) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_CYBERSE as u64)
        && card::is_attack(d, c, &[2300])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and s.spfilter(chkc,e,tp) and not eg:IsContains(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && spfilter(d, chkc, a.e, tp) && !group::is_contains(&eg, chkc);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,eg,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, (&eg).into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,eg,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, (&eg).into()).nil();
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
