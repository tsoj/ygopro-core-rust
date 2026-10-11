//! The Dragon Dwelling in the Deep
//! 海底に潜む深海竜
use super::prelude::*;

pub const ID: u32 = 4404099;

pub const INFO: ScriptInfo = ScriptInfo { counter_list: &[0x23], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(0x23)
    card::enable_counter_permit(d, c, 0x23, None, None);
    // add counter
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e1, CATEGORY_COUNTER);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
    // e1:SetTarget(s.addct)
    effect::set_target(d, e1, Some(cost_target(addct)));
    // e1:SetOperation(s.addc)
    effect::set_operation(d, e1, Some(operation(addc)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // register before leaving
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_LEAVE_FIELD_P)
    effect::set_code(d, e2, EVENT_LEAVE_FIELD_P);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetOperation(s.regop)
    effect::set_operation(d, e2, Some(operation(regop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // increase ATK
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e3, EVENT_LEAVE_FIELD);
    // e3:SetCondition(s.atkcon)
    effect::set_condition(d, e3, Some(condition(atkcon)));
    // e3:SetOperation(s.atkop)
    effect::set_operation(d, e3, Some(operation(atkop)));
    // e3:SetLabelObject(e2)
    effect::set_label_object(d, e3, Value::Effect(e2));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.addct(e,tp,eg,ep,ev,re,r,rp,chk)
fn addct(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_COUNTER,nil,1,0,0x23)
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, None, 1, 0, 0x23);
    true
}
// function s.addc(e,tp,eg,ep,ev,re,r,rp)
fn addc(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():IsRelateToEffect(e) then
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) {
        // e:GetHandler():AddCounter(0x23,1)
        card::add_counter(d, h, 0x23, 1, false);
    }
}
// function s.regop(e,tp,eg,ep,ev,re,r,rp)
fn regop(d: &mut Duel, a: &EffArgs) {
    // local ct=e:GetHandler():GetCounter(0x23)
    let h = effect::get_handler(d, a.e).nil();
    let ct = card::get_counter(d, h, 0x23);
    // e:SetLabel(ct)
    effect::set_label(d, a.e, &[ct as i64]);
}
// function s.atkcon(e,tp,eg,ep,ev,re,r,rp)
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local ct=e:GetLabelObject():GetLabel()
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    let ct = effect::get_label(d, lo);
    // e:SetLabel(ct)
    effect::set_label(d, a.e, &[ct]);
    // return ct>0 and not e:GetHandler():IsLocation(LOCATION_DECK)
    if ct <= 0 {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    !card::is_location(d, h, LOCATION_DECK)
}
// function s.atkop(e,tp,eg,ep,ev,re,r,rp)
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsRace,RACE_FISH|RACE_SEASERPENT),tp,LOCATION_MZONE,0,nil)
    let f = aux::faceup_filter(|d, c| card::is_race(d, c, RACE_FISH | RACE_SEASERPENT));
    let g = duel::get_matching_group(d, Some(&f), a.tp, LOCATION_MZONE, 0, NO_EX);
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(e:GetLabel()*200)
        let lbl = effect::get_label(d, a.e);
        effect::set_value(d, e1, lbl * 200);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
