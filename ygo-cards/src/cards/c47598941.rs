//! Amorphage Lysis
use super::prelude::*;

pub const ID: u32 = 47598941;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_AMORPHAGE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    // e1:SetCondition(aux.StatChangeDamageStepCondition)
    effect::set_condition(d, e1, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // place a card in the pendulum zone
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.setcon)
    effect::set_condition(d, e2, Some(condition(setcon)));
    // e2:SetTarget(s.settg2)
    effect::set_target(d, e2, Some(cost_target(settg2)));
    // e2:SetOperation(s.setop)
    effect::set_operation(d, e2, Some(operation(setop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // atk/def down
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
    // e3:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    // e3:SetTarget(s.atktg)
    effect::set_target(d, e3, Some(effect_card_fn(atktg)));
    // e3:SetValue(s.atkval)
    effect::set_value_fn(d, e3, effect_card_value(atkval));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e4, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsPreviousLocation(LOCATION_PZONE) and c:IsPreviousControler(tp)
    card::is_previous_location(d, c, LOCATION_PZONE) && card::is_previous_controler(d, c, tp)
}
/// `s.setcon(e,tp,eg,ep,ev,re,r,rp)`
fn setcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, a.tp), 1, NO_EX)
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_AMORPHAGE) and c:IsType(TYPE_PENDULUM) and not c:IsForbidden()
    card::is_set_card(d, c, &[SET_AMORPHAGE as u16]) && card::is_type(d, c, TYPE_PENDULUM) && !card::is_forbidden(d, c)
}
/// `s.settg2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckPendulumZones(tp)
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::check_pendulum_zones(d, a.tp) && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // if not Duel.CheckPendulumZones(tp) then return end
    if !duel::check_pendulum_zones(d, a.tp) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOFIELD as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| filter(d, x)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // if tc then
    if let Some(tc) = tc {
        // Duel.MoveToField(tc,tp,tp,LOCATION_PZONE,POS_FACEUP,true)
        duel::move_to_field(d, tc, a.tp, a.tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
    }
}
/// `s.atktg(e,c)`
fn atktg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsSetCard(SET_AMORPHAGE)
    !card::is_set_card(d, c, &[SET_AMORPHAGE as u16])
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, _c: CardId) -> i64 {
    // return Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsSetCard,SET_AMORPHAGE),0,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)*-100
    duel::get_matching_group_count(d, Some(&|d, c| filter_faceup_amorphage(d, c)), 0, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX) as i64 * -100
}
/// `aux.FaceupFilter(Card.IsSetCard,SET_AMORPHAGE)`
fn filter_faceup_amorphage(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_AMORPHAGE as u16])
}
