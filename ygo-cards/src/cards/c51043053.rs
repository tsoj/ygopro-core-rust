//! Heliosphere Dragon
use super::prelude::*;

pub const ID: u32 = 51043053;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK_ANNOUNCE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    // e1:SetCondition(s.atcon)
    effect::set_condition(d, e1, Some(condition(atcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // lv change
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.atcon(e)
fn atcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(e:GetHandlerPlayer(),LOCATION_MZONE,0)==1
    //     and Duel.GetFieldGroupCount(e:GetHandlerPlayer(),0,LOCATION_HAND)<5
    let hp = effect::get_handler_player(d, a.e);
    duel::get_field_group_count(d, hp, LOCATION_MZONE, 0) == 1 && duel::get_field_group_count(d, hp, 0, LOCATION_HAND) < 5
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:GetLevel()==8 and c:IsRace(RACE_DRAGON)
    card::is_faceup(d, c) && card::get_level(d, c) == 8 && card::is_race(d, c, RACE_DRAGON)
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetLevel()~=8
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_MZONE,0,1,nil)
    let h = effect::get_handler(d, a.e).nil();
    card::get_level(d, h) != 8 && duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_LEVEL)
        effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
        // e1:SetValue(8)
        effect::set_value(d, e1, 8);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
