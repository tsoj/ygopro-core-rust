//! Magical Citadel of Endymion
use super::prelude::*;

pub const ID: u32 = 39910367;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    counter_place_list: &[COUNTER_SPELL],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(COUNTER_SPELL)
    card::enable_counter_permit(d, c, COUNTER_SPELL as u16, None, None);
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add counter
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e2:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e2, EVENT_CHAIN_SOLVING);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetOperation(s.op)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Remove counter replace
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_RCOUNTER_REPLACE+COUNTER_SPELL)
    effect::set_code(d, e3, EFFECT_RCOUNTER_REPLACE + COUNTER_SPELL);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(s.rcon)
    effect::set_condition(d, e3, Some(condition(rcon)));
    // e3:SetOperation(s.rop)
    effect::set_operation(d, e3, Some(operation(rop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Destroy replace
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    // e4:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e4, EFFECT_DESTROY_REPLACE);
    // e4:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e4:SetRange(LOCATION_FZONE)
    effect::set_range(d, e4, LOCATION_FZONE);
    // e4:SetTarget(s.desreptg)
    effect::set_target(d, e4, Some(cost_target(desreptg)));
    // e4:SetOperation(s.desrepop)
    effect::set_operation(d, e4, Some(operation(desrepop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Add counter2
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e5, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e5:SetCode(EVENT_LEAVE_FIELD_P)
    effect::set_code(d, e5, EVENT_LEAVE_FIELD_P);
    // e5:SetRange(LOCATION_FZONE)
    effect::set_range(d, e5, LOCATION_FZONE);
    // e5:SetOperation(s.addop2)
    effect::set_operation(d, e5, Some(operation(addop2)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.op(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re);
    // if re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:IsSpellEffect() and rc~=c then
    if effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && effect::is_spell_effect(d, re) && rc != Some(c) {
        // c:AddCounter(COUNTER_SPELL,1)
        card::add_counter(d, c, COUNTER_SPELL as u16, 1, false);
    }
}
/// `s.rcon(e,tp,eg,ep,ev,re,r,rp)`
fn rcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return (r&REASON_COST)~=0 and re:IsActivated() and ep==e:GetOwnerPlayer() and c:GetCounter(COUNTER_SPELL)>=ev
    //     and re:GetHandler()~=c
    let re = a.re.nil();
    (a.r & REASON_COST) != 0
        && effect::is_activated(d, re)
        && a.ep == effect::get_owner_player(d, a.e)
        && card::get_counter(d, c, COUNTER_SPELL as u16) as i64 >= a.ev as i64
        && effect::get_handler(d, re) != Some(c)
}
/// `s.rop(e,tp,eg,ep,ev,re,r,rp)`
fn rop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():RemoveCounter(ep,COUNTER_SPELL,ev,REASON_EFFECT)
    let c = effect::get_handler(d, a.e).nil();
    card::remove_counter(d, c, a.ep, COUNTER_SPELL as u16, a.ev as u16, REASON_EFFECT);
}
/// `s.desreptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn desreptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return not e:GetHandler():IsReason(REASON_REPLACE+REASON_RULE)
    //     and e:GetHandler():GetCounter(COUNTER_SPELL)>0 end
    if a.chk == Some(0) {
        return !card::is_reason(d, c, REASON_REPLACE + REASON_RULE) && card::get_counter(d, c, COUNTER_SPELL as u16) > 0;
    }
    // return Duel.SelectEffectYesNo(tp,e:GetHandler(),96)
    duel::select_effect_yes_no(d, a.tp, c, Some(96))
}
/// `s.desrepop(e,tp,eg,ep,ev,re,r,rp)`
fn desrepop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():RemoveCounter(ep,COUNTER_SPELL,1,REASON_EFFECT)
    let c = effect::get_handler(d, a.e).nil();
    card::remove_counter(d, c, a.ep, COUNTER_SPELL as u16, 1, REASON_EFFECT);
}
/// `s.addop2(e,tp,eg,ep,ev,re,r,rp)`
fn addop2(d: &mut Duel, a: &EffArgs) {
    // local count=0
    let mut count: i64 = 0;
    // for c in aux.Next(eg) do
    for c in group::iter(d, &a.eg()) {
        // if not c:IsCode(id) and c:IsLocation(LOCATION_ONFIELD) and c:IsReason(REASON_DESTROY) then
        if !card::is_code(d, c, &[ID]) && card::is_location(d, c, LOCATION_ONFIELD) && card::is_reason(d, c, REASON_DESTROY) {
            // count=count+c:GetCounter(COUNTER_SPELL)
            count += card::get_counter(d, c, COUNTER_SPELL as u16) as i64;
        }
    }
    // if count>0 then
    if count > 0 {
        // e:GetHandler():AddCounter(COUNTER_SPELL,count)
        let c = effect::get_handler(d, a.e).nil();
        card::add_counter(d, c, COUNTER_SPELL as u16, count as u16, false);
    }
}
