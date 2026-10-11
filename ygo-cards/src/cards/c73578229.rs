//! Pole Position
use super::prelude::*;

pub const ID: u32 = 73578229;

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
    // local g=Group.CreateGroup()
    // g:KeepAlive()
    let g = CardSet::new();
    // Adjust
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetCode(EVENT_ADJUST)
    effect::set_code(d, e2, EVENT_ADJUST);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetOperation(s.adjustop)
    effect::set_operation(d, e2, Some(operation(adjustop)));
    // e2:SetLabelObject(g)
    // (the group lives in the label object of e2; e3 reaches it through e2)
    effect::set_label_object(d, e2, Value::Group(g));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
    // e3:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e3, EFFECT_IMMUNE_EFFECT);
    // e3:SetTarget(s.etarget)
    effect::set_target(d, e3, Some(effect_card_fn(etarget)));
    // e3:SetValue(s.efilter)
    effect::set_value_fn(d, e3, lua_fn(efilter));
    // e3:SetLabelObject(g)
    effect::set_label_object(d, e3, Value::Effect(e2));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Register before leaving
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    // e4:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e4, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e4:SetCode(EVENT_LEAVE_FIELD_P)
    effect::set_code(d, e4, EVENT_LEAVE_FIELD_P);
    // e4:SetOperation(s.checkop)
    effect::set_operation(d, e4, Some(operation(checkop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Destroy highest ATK monster
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e5:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e5, EVENT_LEAVE_FIELD);
    // e5:SetLabelObject(e4)
    effect::set_label_object(d, e5, Value::Effect(e4));
    // e5:SetOperation(s.desop)
    effect::set_operation(d, e5, Some(operation(desop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// The group of the card that `e` (e3) reaches through its label object (e2).
fn shared_group(d: &mut Duel, e: EffectId) -> CardSet {
    // e:GetLabelObject()
    let e2 = effect::get_label_object(d, e).as_effect().nil();
    // e2:GetLabelObject()
    effect::get_label_object(d, e2).as_group().cloned().nil()
}
/// `s.etarget(e,c)`
fn etarget(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return e:GetLabelObject():IsContains(c)
    let g = shared_group(d, e);
    group::is_contains(&g, c)
}
/// `s.efilter(e,te)`
fn efilter(d: &mut Duel, a: Args) -> bool {
    // return te:IsSpellEffect()
    effect::is_spell_effect(d, a.e(1))
}
/// `s.adjustop(e,tp,eg,ep,ev,re,r,rp)`
fn adjustop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // local preg=e:GetLabelObject()
    let mut preg = effect::get_label_object(d, a.e).as_group().cloned().nil();
    // if #g>0 then
    if !g.is_empty() {
        // local ag=g:GetMaxGroup(Card.GetAttack)
        let ag = group::get_max_group(d, &g, &|d, x| card::get_attack(d, x) as i64).map_or_else(CardSet::new, |x| x.0);
        // if ag:Equal(preg) then return end
        if group::equal(&ag, &preg) {
            return;
        }
        // preg:Clear()
        preg.clear();
        // preg:Merge(ag)
        group::add_card(&mut preg, &ag);
    } else {
        // if #preg==0 then return end
        if preg.is_empty() {
            return;
        }
        // preg:Clear()
        preg.clear();
    }
    // the group is stored back in the label object
    effect::set_label_object(d, a.e, Value::Group(preg));
    // Duel.AdjustInstantly(e:GetHandler())
    let handler = effect::get_handler(d, a.e).nil();
    duel::adjust_instantly(d, Some(handler));
    // Duel.Readjust()
    duel::readjust(d);
}
/// `s.checkop(e,tp,eg,ep,ev,re,r,rp)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsDisabled() or not c:IsStatus(STATUS_EFFECT_ENABLED) then
    if card::is_disabled(d, c) || !card::is_status(d, c, STATUS_EFFECT_ENABLED) {
        // e:SetLabel(1)
        effect::set_label(d, a.e, &[1]);
    } else {
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
    }
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // if e:GetLabelObject():GetLabel()==0 then
    let e4 = effect::get_label_object(d, a.e).as_effect().nil();
    if effect::get_label(d, e4) == 0 {
        // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
        // if #g>0 then
        if !g.is_empty() {
            // local ag=g:GetMaxGroup(Card.GetAttack)
            let ag = group::get_max_group(d, &g, &|d, x| card::get_attack(d, x) as i64).map_or_else(CardSet::new, |x| x.0);
            // Duel.Destroy(ag,REASON_EFFECT)
            duel::destroy(d, ag, REASON_EFFECT, None, None);
        }
    }
}
