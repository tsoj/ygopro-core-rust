//! "A" Cell Incubator
use super::prelude::*;

pub const ID: u32 = 64163367;

pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[COUNTER_A], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // counter1
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EVENT_REMOVE_COUNTER+COUNTER_A)
    effect::set_code(d, e2, EVENT_REMOVE_COUNTER + COUNTER_A);
    // e2:SetOperation(s.ctop1)
    effect::set_operation(d, e2, Some(operation(ctop1)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // register before leaving
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_LEAVE_FIELD_P)
    effect::set_code(d, e3, EVENT_LEAVE_FIELD_P);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e3:SetOperation(s.regop)
    effect::set_operation(d, e3, Some(operation(regop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // place counters when leaving
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e4, CATEGORY_COUNTER);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e4:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e4, EVENT_DESTROYED);
    // e4:SetCondition(s.ctcon2)
    effect::set_condition(d, e4, Some(condition(ctcon2)));
    // e4:SetOperation(s.ctop2)
    effect::set_operation(d, e4, Some(operation(ctop2)));
    // e4:SetLabelObject(e3)
    effect::set_label_object(d, e4, Value::Effect(e3));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// function s.ctop1(e,tp,eg,ep,ev,re,r,rp)
fn ctop1(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():AddCounter(COUNTER_NEED_ENABLE+COUNTER_A,1)
    let h = effect::get_handler(d, a.e).nil();
    card::add_counter(d, h, (COUNTER_NEED_ENABLE + COUNTER_A) as u16, 1, false);
}
// function s.regop(e,tp,eg,ep,ev,re,r,rp)
fn regop(d: &mut Duel, a: &EffArgs) {
    // local ct=e:GetHandler():GetCounter(COUNTER_A)
    let h = effect::get_handler(d, a.e).nil();
    let ct = card::get_counter(d, h, COUNTER_A as u16);
    // e:SetLabel(ct)
    effect::set_label(d, a.e, &[ct as i64]);
}
// function s.ctcon2(e,tp,eg,ep,ev,re,r,rp)
fn ctcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // local ct=e:GetLabelObject():GetLabel()
    let e3 = effect::get_label_object(d, a.e).as_effect().nil();
    let ct = effect::get_label(d, e3);
    // e:SetLabel(ct)
    effect::set_label(d, a.e, &[ct]);
    // return ct>0
    ct > 0
}
// function s.ctop2(e,tp,eg,ep,ev,re,r,rp)
fn ctop2(d: &mut Duel, a: &EffArgs) {
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, a.e);
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if #g==0 then return end
    if group::get_count(&g) == 0 {
        return;
    }
    // for i=1,ct do
    for _ in 0..ct {
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 1));
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        // sg:GetFirst():AddCounter(COUNTER_A,1)
        let tc = group::get_first(d, &sg).nil();
        card::add_counter(d, tc, COUNTER_A as u16, 1, false);
    }
}
