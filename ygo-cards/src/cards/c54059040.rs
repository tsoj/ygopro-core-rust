//! Eisbahn
use super::prelude::*;

pub const ID: u32 = 54059040;

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
    // Change non-WATER monsters to Defense Position
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_POSITION)
    effect::set_category(d, e2, CATEGORY_POSITION);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(s.poscon)
    effect::set_condition(d, e2, Some(condition(poscon)));
    // e2:SetTarget(s.postg)
    effect::set_target(d, e2, Some(cost_target(postg)));
    // e2:SetOperation(s.posop)
    effect::set_operation(d, e2, Some(operation(posop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// s.poscon(e,tp,eg,ep,ev,re,r,rp)
fn poscon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsAttribute,ATTRIBUTE_WATER),tp,LOCATION_MZONE,0,1,nil)
    //     and eg:IsExists(aux.FaceupFilter(Card.IsAttributeExcept,ATTRIBUTE_WATER),1,nil)
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_WATER)), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
        && group::is_exists(d, &a.eg.clone().nil(), &|d, c| card::is_faceup(d, c) && card::is_attribute_except(d, c, ATTRIBUTE_WATER, None, 0, None), 1, NO_EX)
}
// s.postg(e,tp,eg,ep,ev,re,r,rp,chk)
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local posg=eg:Filter(aux.FaceupFilter(Card.IsAttributeExcept,ATTRIBUTE_WATER),nil)
    let eg = a.eg.clone().nil();
    let posg = group::filter(d, &eg, &|d, c| card::is_faceup(d, c) && card::is_attribute_except(d, c, ATTRIBUTE_WATER, None, 0, None), NO_EX);
    // Duel.SetTargetCard(posg)
    duel::set_target_card(d, posg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,posg,#posg,tp,0)
    let n = posg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(posg), n, a.tp, 0);
    true
}
// s.posop(e,tp,eg,ep,ev,re,r,rp)
fn posop(d: &mut Duel, a: &EffArgs) {
    // local posg=Duel.GetTargetCards(e)
    let posg = duel::get_target_cards(d, a.e).nil();
    // if #posg>0 then
    if !posg.is_empty() {
        // Duel.ChangePosition(posg,POS_FACEUP_DEFENSE)
        duel::change_position(d, posg, POS_FACEUP_DEFENSE as u8, None, None, None, false);
    }
}
