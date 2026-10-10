//! Element Soldier
use super::prelude::*;

pub const ID: u32 = 66712593;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // control
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_CANNOT_CHANGE_CONTROL)
    effect::set_code(d, e1, EFFECT_CANNOT_CHANGE_CONTROL);
    // e1:SetCondition(s.ctlcon)
    effect::set_condition(d, e1, Some(condition(ctlcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // disable
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_BATTLED)
    effect::set_code(d, e2, EVENT_BATTLED);
    // e2:SetCondition(s.discon)
    effect::set_condition(d, e2, Some(condition(discon)));
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.filter(c,att)`
fn filter(d: &mut Duel, c: CardId, att: u32) -> bool {
    // return c:IsFaceup() and c:IsAttribute(att)
    card::is_faceup(d, c) && card::is_attribute(d, c, att)
}
/// `s.ctlcon(e)`
fn ctlcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.filter,0,LOCATION_MZONE,LOCATION_MZONE,1,nil,ATTRIBUTE_WATER)
    duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, ATTRIBUTE_WATER)), 0, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    // return bc and bc:IsStatus(STATUS_BATTLE_DESTROYED) and not c:IsStatus(STATUS_BATTLE_DESTROYED)
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil,ATTRIBUTE_EARTH)
    let Some(bc) = card::get_battle_target(d, c) else { return false };
    card::is_status(d, bc, STATUS_BATTLE_DESTROYED)
        && !card::is_status(d, c, STATUS_BATTLE_DESTROYED)
        && duel::is_existing_matching_card(d, Some(&|d, x| filter(d, x, ATTRIBUTE_EARTH)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_DISABLE)
    effect::set_code(d, e1, EFFECT_DISABLE);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD_EXC_GRAVE)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_EXC_GRAVE, 1);
    // bc:RegisterEffect(e1)
    card::register_effect(d, bc, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_DISABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
    // e2:SetReset(RESET_EVENT|RESETS_STANDARD_EXC_GRAVE)
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD_EXC_GRAVE, 1);
    // bc:RegisterEffect(e2)
    card::register_effect(d, bc, e2, false);
}
