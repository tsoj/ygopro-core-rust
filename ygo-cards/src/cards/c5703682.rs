//! Thousand Energy
use super::prelude::*;

pub const ID: u32 = 5703682;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // local tpe=c:GetType()
    let tpe = card::get_type(d, c);
    // return c:IsFaceup() and (tpe&TYPE_NORMAL)~=0 and (tpe&TYPE_TOKEN)==0 and c:GetLevel()==2
    card::is_faceup(d, c) && (tpe & TYPE_NORMAL) != 0 && (tpe & TYPE_TOKEN) == 0 && card::get_level(d, c) == 2
}
// s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    true
}
// s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&filter), a.tp, LOCATION_MZONE, 0, NO_EX);
    // local tc=g:GetFirst()
    let _ = group::get_first(d, &g);
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_BASE_ATTACK)
        effect::set_code(d, e1, EFFECT_SET_BASE_ATTACK);
        // e1:SetValue(tc:GetBaseAttack()+1000)
        let base_attack = card::get_base_attack(d, tc);
        effect::set_value(d, e1, base_attack as i64 + 1000);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=Effect.CreateEffect(e:GetHandler())
        let e2 = effect::create_effect(d, h);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_SET_BASE_DEFENSE)
        effect::set_code(d, e2, EFFECT_SET_BASE_DEFENSE);
        // e2:SetValue(tc:GetBaseDefense()+1000)
        let base_defense = card::get_base_defense(d, tc);
        effect::set_value(d, e2, base_defense as i64 + 1000);
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
    }
    // local de=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let de = effect::create_effect(d, h);
    // de:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, de, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // de:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, de, EVENT_PHASE + PHASE_END);
    // de:SetCountLimit(1)
    effect::set_count_limit(d, de, 1, 0, 0, 0);
    // de:SetCondition(s.descon)
    effect::set_condition(d, de, Some(condition(descon)));
    // de:SetOperation(s.desop)
    effect::set_operation(d, de, Some(operation(desop)));
    // de:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, de, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(de,tp)
    duel::register_effect(d, de, a.tp);
}
// s.dfilter(c)
fn dfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_NORMAL) and c:GetLevel()==2
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_NORMAL) && card::get_level(d, c) == 2
}
// s.descon(e,tp,eg,ep,ev,re,r,rp)
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.dfilter,tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&dfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
// s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.dfilter,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&dfilter), a.tp, LOCATION_MZONE, 0, NO_EX);
    // Duel.Destroy(g,REASON_EFFECT)
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
