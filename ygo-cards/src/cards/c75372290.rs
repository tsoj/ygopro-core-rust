//! Total Defense Shogun
use super::prelude::*;

pub const ID: u32 = 75372290;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // to defense
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_POSITION)
    effect::set_category(d, e1, CATEGORY_POSITION);
    // e1:SetType(EFFECT_TYPE_TRIGGER_F+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_TRIGGER_F + EFFECT_TYPE_SINGLE);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetTarget(s.postg)
    effect::set_target(d, e1, Some(cost_target(postg)));
    // e1:SetOperation(s.posop)
    effect::set_operation(d, e1, Some(operation(posop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_FLIP_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_FLIP_SUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_DEFENSE_ATTACK)
    effect::set_code(d, e3, EFFECT_DEFENSE_ATTACK);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetCode(id)
    effect::set_code(d, e4, ID);
    // e4:SetCondition(s.dacon)
    effect::set_condition(d, e4, Some(condition(dacon)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// function s.dacon(e)
fn dacon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsDefensePos()
    let h = effect::get_handler(d, a.e).nil();
    card::is_defense_pos(d, h)
}
// function s.postg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():IsAttackPos() end
    if a.chk == Some(0) {
        return card::is_attack_pos(d, h);
    }
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
// function s.posop(e,tp,eg,ep,ev,re,r,rp)
fn posop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFaceup() and c:IsAttackPos() and c:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_attack_pos(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // Duel.ChangePosition(c,POS_FACEUP_DEFENSE)
        duel::change_position(d, c, POS_FACEUP_DEFENSE as u8, None, None, None, false);
    }
}
