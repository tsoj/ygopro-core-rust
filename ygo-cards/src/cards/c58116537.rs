//! Ancient Warriors Saga - Sun-Liu Alliance
use super::prelude::*;

pub const ID: u32 = 58116537;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ANCIENT_WARRIORS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Declare attribute
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.nacon)
    effect::set_condition(d, e2, Some(condition(nacon)));
    // e2:SetTarget(s.natg)
    effect::set_target(d, e2, Some(cost_target(natg)));
    // e2:SetOperation(s.naop)
    effect::set_operation(d, e2, Some(operation(naop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Increase ATK
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e3, CATEGORY_ATKCHANGE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(s.atkcon1)
    effect::set_condition(d, e3, Some(condition(atkcon1)));
    // e3:SetTarget(s.atktg)
    effect::set_target(d, e3, Some(cost_target(atktg)));
    // e3:SetOperation(s.atkop)
    effect::set_operation(d, e3, Some(operation(atkop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e5=e3:Clone()
    let e5 = effect::clone(d, e3);
    // e5:SetCode(EVENT_CHAINING)
    effect::set_code(d, e5, EVENT_CHAINING);
    // e5:SetCondition(s.atkcon2)
    effect::set_condition(d, e5, Some(condition(atkcon2)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
// function s.nacon(e,tp,eg,ep,ev,re,r,rp)
fn nacon(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsSetCard,SET_ANCIENT_WARRIORS),tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_ANCIENT_WARRIORS as u16]))), a.tp, LOCATION_MZONE, 0, NO_EX);
    // return #g>1 and g:GetClassCount(Card.GetAttribute)>1
    g.len() > 1 && group::get_class_count(d, &g, &|d, c| card::get_attribute(d, c) as i64) > 1
}
// function s.natg(e,tp,eg,ep,ev,re,r,rp,chk)
fn natg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsFaceup,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // local att=0
    let mut att: u32 = 0;
    // local t=g:GetClass(Card.GetAttribute)
    let t = group::get_class(d, &g, &|d, c| card::get_attribute(d, c) as i64);
    // for _,i in ipairs(t) do att=att|i end
    for i in t {
        att |= i as u32;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATTRIBUTE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATTRIBUTE as u64);
    // local rc=Duel.AnnounceAttribute(tp,1,att)
    let rc = duel::announce_attribute(d, a.tp, 1, att);
    // e:SetLabel(rc)
    effect::set_label(d, a.e, &[rc as i64]);
    true
}
// function s.naop(e,tp,eg,ep,ev,re,r,rp,chk)
fn naop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local att=e:GetLabel()
    let att = effect::get_label(d, a.e) as u32;
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsAttribute,att),tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&aux::faceup_filter(|d, c| card::is_attribute(d, c, att))), a.tp, 0, LOCATION_MZONE, NO_EX);
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CANNOT_TRIGGER)
        effect::set_code(d, e1, EFFECT_CANNOT_TRIGGER);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1,true)
        card::register_effect(d, tc, e1, true);
    }
}
// function s.atkcon1(e,tp,eg,ep,ev,re,r,rp)
fn atkcon1(_d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==1-tp
    a.rp == 1 - a.tp
}
// function s.atkcon2(e,tp,eg,ep,ev,re,r,rp)
fn atkcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==tp and re:GetHandler():IsSetCard(SET_ANCIENT_WARRIORS) and re:GetHandler():IsMonster()
    if a.rp != a.tp {
        return false;
    }
    let rc = effect::get_handler(d, a.re.nil()).nil();
    card::is_set_card(d, rc, &[SET_ANCIENT_WARRIORS as u16]) && {
        let rc = effect::get_handler(d, a.re.nil()).nil();
        card::is_monster(d, rc)
    }
}
// function s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_ANCIENT_WARRIORS),tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_ANCIENT_WARRIORS as u16]))), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    true
}
// function s.atkop(e,tp,eg,ep,ev,re,r,rp)
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsSetCard,SET_ANCIENT_WARRIORS),tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_ANCIENT_WARRIORS as u16]))), a.tp, LOCATION_MZONE, 0, NO_EX);
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // tc:UpdateAttack(#g*300,RESETS_STANDARD_PHASE_END,c)
        card::update_attack(d, tc, g.len() as i64 * 300, Some(RESETS_STANDARD_PHASE_END), Some(c), None);
    }
}
