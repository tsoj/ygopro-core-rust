//! Field Power Bonus
use super::prelude::*;

pub const ID: u32 = 100459016;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1a, aux::stringid(ID, 0));
    // e1a:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE)
    effect::set_category(d, e1a, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    // e1a:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1a, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e1a:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1a, EVENT_SUMMON_SUCCESS);
    // e1a:SetRange(LOCATION_FZONE)
    effect::set_range(d, e1a, LOCATION_FZONE);
    // e1a:SetCountLimit(1,0,EFFECT_COUNT_CODE_CHAIN)
    effect::set_count_limit(d, e1a, 1, 0, 0, EFFECT_COUNT_CODE_CHAIN as u8);
    // e1a:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return eg:IsExists(Card.IsFaceup,1,nil) end)
    effect::set_condition(d, e1a, Some(condition(e1a_cond)));
    // e1a:SetTarget(s.atktg)
    effect::set_target(d, e1a, Some(cost_target(atktg)));
    // e1a:SetOperation(s.atkop)
    effect::set_operation(d, e1a, Some(operation(atkop)));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1b, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DISABLE+CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE)
    effect::set_category(d, e2, CATEGORY_DISABLE + CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetTarget(s.distg)
    effect::set_target(d, e2, Some(cost_target(distg)));
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_LEAVE_GRAVE)
    effect::set_category(d, e3, CATEGORY_LEAVE_GRAVE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_PHASE+PHASE_STANDBY)
    effect::set_code(d, e3, EVENT_PHASE + PHASE_STANDBY);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return Duel.GetFieldGroupCount(tp,LOCATION_FZONE,LOCATION_FZONE)==0 end)
    effect::set_condition(d, e3, Some(condition(|d, a| duel::get_field_group_count(d, a.tp, LOCATION_FZONE, LOCATION_FZONE) == 0)));
    // e3:SetTarget(s.pltg)
    effect::set_target(d, e3, Some(cost_target(pltg)));
    // e3:SetOperation(s.plop)
    effect::set_operation(d, e3, Some(operation(plop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.typecheckfilter(c)`
fn typecheckfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,c:GetRace()),0,LOCATION_MZONE,LOCATION_MZONE,1,c)
    if !card::is_faceup(d, c) {
        return false;
    }
    let race = card::get_race(d, c);
    duel::is_existing_matching_card(
        d,
        Some(&|d, x| card::is_faceup(d, x) && card::is_race(d, x, race)),
        0,
        LOCATION_MZONE,
        LOCATION_MZONE,
        1,
        c.into(),
    )
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return eg:IsExists(Card.IsFaceup,1,nil) end`
fn e1a_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, x| card::is_faceup(d, x), 1, NO_EX)
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    // if chk==0 then return eg:IsExists(s.typecheckfilter,1,nil) end
    if a.chk == Some(0) {
        return group::is_exists(d, &eg, &typecheckfilter, 1, NO_EX);
    }
    // local sg=eg:Filter(s.typecheckfilter,nil)
    let sg = group::filter(d, &eg, &typecheckfilter, NO_EX);
    // for sc in sg:Iter() do sc:CreateEffectRelation(e) end
    for sc in group::iter(d, &sg) {
        card::create_effect_relation(d, sc, a.e);
    }
    // e:GetChainData().summon_group=sg
    chain::set_effect_data(d, a.e, None, chain::field::CARD, Value::Group(sg.clone()));
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,sg,#sg,tp,1000)
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(sg.clone()), sg.len() as u8, a.tp, 1000);
    // Duel.SetOperationInfo(0,CATEGORY_DEFCHANGE,sg,#sg,tp,1000)
    duel::set_operation_info(d, 0, CATEGORY_DEFCHANGE, Some(sg.clone()), sg.len() as u8, a.tp, 1000);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local sg=e:GetChainData().summon_group:Match(aux.FaceupFilter(Card.IsRelateToEffect,e),nil)
    let mut sg = chain::effect_data(d, a.e, None, chain::field::CARD).as_group().cloned().nil();
    let e = a.e;
    group::match_(d, &mut sg, &|d, x| card::is_faceup(d, x) && card::is_relate_to_effect(d, x, e), NO_EX);
    // for tc in sg:Iter() do
    for tc in group::iter(d, &sg) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(1000)
        effect::set_value(d, e1, 1000);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_UPDATE_DEFENSE)
        effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
    }
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,g,#g,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(g.clone()), g.len() as u8, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,g,#g,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(g.clone()), g.len() as u8, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DEFCHANGE,g,#g,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DEFCHANGE, Some(g.clone()), g.len() as u8, a.tp, 0);
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // if tc:IsNegatableMonster() then tc:NegateEffects(c) end
        if card::is_negatable_monster(d, tc) {
            card::negate_effects(d, tc, c, None, false, None);
        }
        // if tc:GetAttack()~=tc:GetBaseAttack() then
        let atk = card::get_attack(d, tc);
        let base_atk = card::get_base_attack(d, tc);
        if atk != base_atk {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
            effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
            // e1:SetValue(tc:GetBaseAttack())
            effect::set_value(d, e1, base_atk as i64);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
        // if tc:GetDefense()~=tc:GetBaseDefense() then
        let def = card::get_defense(d, tc);
        let base_def = card::get_base_defense(d, tc);
        if def != base_def {
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetCode(EFFECT_SET_DEFENSE_FINAL)
            effect::set_code(d, e2, EFFECT_SET_DEFENSE_FINAL);
            // e2:SetValue(tc:GetBaseDefense())
            effect::set_value(d, e2, base_def as i64);
            // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
        }
    }
}
/// `s.pltg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn pltg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:CheckUniqueOnField(tp) end
    if a.chk == Some(0) {
        return card::check_unique_on_field(d, c, a.tp, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.plop(e,tp,eg,ep,ev,re,r,rp)`
fn plop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.MoveToField(c,tp,tp,LOCATION_FZONE,POS_FACEUP,true)
        duel::move_to_field(d, c, a.tp, a.tp, LOCATION_FZONE as u16, POS_FACEUP as u8, true, None);
    }
}
