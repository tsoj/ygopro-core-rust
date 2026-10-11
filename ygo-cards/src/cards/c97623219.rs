//! Element Valkyrie
use super::prelude::*;

pub const ID: u32 = 97623219;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atk
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(500)
    effect::set_value(d, e1, 500);
    // e1:SetCondition(s.atkcon)
    effect::set_condition(d, e1, Some(condition(atkcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // control
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EFFECT_CANNOT_CHANGE_CONTROL)
    effect::set_code(d, e2, EFFECT_CANNOT_CHANGE_CONTROL);
    // e2:SetCondition(s.ctlcon)
    effect::set_condition(d, e2, Some(condition(ctlcon)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.filter(c,att)
fn filter(d: &mut Duel, c: CardId, att: u32) -> bool {
    // return c:IsFaceup() and c:IsAttribute(att)
    card::is_faceup(d, c) && card::is_attribute(d, c, att)
}
// function s.atkcon(e)
fn atkcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.filter,0,LOCATION_MZONE,LOCATION_MZONE,1,nil,ATTRIBUTE_FIRE)
    duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, ATTRIBUTE_FIRE)), 0, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
}
// function s.ctlcon(e)
fn ctlcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.filter,0,LOCATION_MZONE,LOCATION_MZONE,1,nil,ATTRIBUTE_WATER)
    duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, ATTRIBUTE_WATER)), 0, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
}
