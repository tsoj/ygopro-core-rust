//! Lemuria, the Forgotten City
use super::prelude::*;

pub const ID: u32 = 34103656;

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
    // Increase ATK/DEF
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsAttribute,ATTRIBUTE_WATER))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_attribute(d, c, ATTRIBUTE_WATER))));
    // e2:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    // e2:SetValue(200)
    effect::set_value(d, e2, 200);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Increase Level
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_LVCHANGE)
    effect::set_category(d, e4, CATEGORY_LVCHANGE);
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetRange(LOCATION_FZONE)
    effect::set_range(d, e4, LOCATION_FZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetTarget(s.lvtg)
    effect::set_target(d, e4, Some(cost_target(lvtg)));
    // e4:SetOperation(s.lvop)
    effect::set_operation(d, e4, Some(operation(lvop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAttribute(ATTRIBUTE_WATER) and c:HasLevel()
    card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_WATER) && card::has_level(d, c)
}
/// `s.lvtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn lvtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    true
}
/// `s.lvop(e,tp,eg,ep,ev,re,r,rp)`
fn lvop(d: &mut Duel, a: &EffArgs) {
    // local lv=Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsAttribute,ATTRIBUTE_WATER),tp,LOCATION_MZONE,0,nil)
    let lv = duel::get_matching_group_count(
        d,
        Some(&|d, c| card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_WATER)),
        a.tp,
        LOCATION_MZONE,
        0,
        NO_EX,
    );
    // if lv==0 then return end
    if lv == 0 {
        return;
    }
    // local g=Duel.GetFieldGroup(tp,LOCATION_MZONE,0)
    let g = duel::get_field_group(d, a.tp, LOCATION_MZONE, 0);
    // local _,fid=g:GetMaxGroup(Card.GetFieldID)
    let fid = group::get_max_group(d, &g, &|d, c| card::get_field_id(d, c) as i64).map_or(0, |r| r.1);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Increase Level
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_LEVEL)
    effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
    // e1:SetRange(LOCATION_FZONE)
    effect::set_range(d, e1, LOCATION_FZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(effect_card_fn(efftg)));
    // e1:SetValue(lv)
    effect::set_value(d, e1, lv as i64);
    // e1:SetLabel(fid)
    effect::set_label(d, e1, &[fid]);
    // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.efftg(e,c)`
fn efftg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:GetFieldID()<=e:GetLabel() and c:IsAttribute(ATTRIBUTE_WATER) and c:HasLevel()
    (card::get_field_id(d, c) as i64) <= effect::get_label(d, e)
        && card::is_attribute(d, c, ATTRIBUTE_WATER)
        && card::has_level(d, c)
}
