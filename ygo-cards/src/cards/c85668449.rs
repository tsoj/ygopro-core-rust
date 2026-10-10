//! Brain Research Lab
use super::prelude::*;

pub const ID: u32 = 85668449;

pub const INFO: ScriptInfo = ScriptInfo { counter_list: &[0x4], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(0x4)
    card::enable_counter_permit(d, c, 0x4, None, None);
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Extra summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetTargetRange(LOCATION_HAND|LOCATION_MZONE,LOCATION_HAND|LOCATION_MZONE)
    effect::set_target_range(d, e2, LOCATION_HAND | LOCATION_MZONE, LOCATION_HAND | LOCATION_MZONE);
    // e2:SetCode(EFFECT_EXTRA_SUMMON_COUNT)
    effect::set_code(d, e2, EFFECT_EXTRA_SUMMON_COUNT);
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsRace,RACE_PSYCHIC))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_race(d, c, RACE_PSYCHIC))));
    // e2:SetOperation(s.esop)
    effect::set_operation(d, e2, Some(lua_fn(esop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SUMMON_SUCCESS);
    // e3:SetCondition(s.ctcon)
    effect::set_condition(d, e3, Some(condition(ctcon)));
    // e3:SetOperation(s.ctop)
    effect::set_operation(d, e3, Some(operation(ctop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // LP Cost replace
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e4:SetRange(LOCATION_FZONE)
    effect::set_range(d, e4, LOCATION_FZONE);
    // e4:SetCode(EFFECT_LPCOST_REPLACE)
    effect::set_code(d, e4, EFFECT_LPCOST_REPLACE);
    // e4:SetCondition(s.lrcon)
    effect::set_condition(d, e4, Some(condition(lrcon)));
    // e4:SetOperation(s.lrop)
    effect::set_operation(d, e4, Some(operation(lrop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Inflict Damage
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e5:SetCode(EVENT_LEAVE_FIELD_P)
    effect::set_code(d, e5, EVENT_LEAVE_FIELD_P);
    // e5:SetOperation(s.damp)
    effect::set_operation(d, e5, Some(operation(damp)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e6, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e6:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e6, EVENT_LEAVE_FIELD);
    // e6:SetOperation(s.damop)
    effect::set_operation(d, e6, Some(operation(damop)));
    // e6:SetLabelObject(e5)
    effect::set_label_object(d, e6, Value::Effect(e5));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
/// `s.esop(e,tp,eg,ep,ev,re,r,rp,c)`
fn esop(d: &mut Duel, a: Args) {
    let c = a.c(8);
    // c:RegisterFlagEffect(id,RESET_EVENT|(RESETS_STANDARD_PHASE_END&~RESET_TOFIELD),0,1)
    card::register_flag_effect(d, c, ID, RESET_EVENT | (RESETS_STANDARD_PHASE_END & !RESET_TOFIELD), 0, 1, 0, 0);
}
/// `s.ctcon(e,tp,eg,ep,ev,re,r,rp)`
fn ctcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:GetFirst():GetFlagEffect(id)~=0
    let f = group::get_first(d, &a.eg.clone().nil()).nil();
    card::get_flag_effect(d, f, ID) != 0
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():AddCounter(0x4,1)
    let h = effect::get_handler(d, a.e).nil();
    card::add_counter(d, h, 0x4, 1, false);
    // eg:GetFirst():ResetFlagEffect(id)
    let f = group::get_first(d, &a.eg.clone().nil()).nil();
    card::reset_flag_effect(d, f, ID);
}
/// `s.lrcon(e,tp,eg,ep,ev,re,r,rp)`
fn lrcon(d: &mut Duel, a: &EffArgs) -> bool {
    // if tp~=ep then return false end
    if a.tp != a.ep {
        return false;
    }
    // if Duel.GetLP(ep)<ev then return false end
    if (duel::get_lp(d, a.ep) as i64) < a.ev as i64 {
        return false;
    }
    // if not (re and re:IsActivated()) then return false end
    let Some(re) = a.re else {
        return false;
    };
    if !effect::is_activated(d, re) {
        return false;
    }
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, re).nil();
    // return rc:IsLocation(LOCATION_MZONE) and rc:IsRace(RACE_PSYCHIC)
    card::is_location(d, rc, LOCATION_MZONE) && card::is_race(d, rc, RACE_PSYCHIC)
}
/// `s.lrop(e,tp,eg,ep,ev,re,r,rp)`
fn lrop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():AddCounter(0x4,1)
    let h = effect::get_handler(d, a.e).nil();
    card::add_counter(d, h, 0x4, 1, false);
}
/// `s.damp(e,tp,eg,ep,ev,re,r,rp)`
fn damp(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ct=c:GetCounter(0x4)
    let ct = card::get_counter(d, c, 0x4);
    // e:SetLabel(ct)
    effect::set_label(d, a.e, &[ct as i64]);
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    // local ct=e:GetLabelObject():GetLabel()
    let e5 = effect::get_label_object(d, a.e).as_effect().nil();
    let ct = effect::get_label(d, e5);
    // if ct>0 then
    if ct > 0 {
        // Duel.Damage(tp,ct*1000,REASON_EFFECT)
        duel::damage(d, a.tp, ct * 1000, REASON_EFFECT, false, None);
    }
}
