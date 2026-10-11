//! Spider Web
//! スパイダー・ウェブ
use super::prelude::*;

pub const ID: u32 = 69408987;

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
    // to defense
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e2, EVENT_DAMAGE_STEP_END);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCondition(s.poscon)
    effect::set_condition(d, e2, Some(condition(poscon)));
    // e2:SetOperation(s.posop)
    effect::set_operation(d, e2, Some(operation(posop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// s.poscon(e,tp,eg,ep,ev,re,r,rp)
fn poscon(d: &mut Duel, _a: &EffArgs) -> bool {
    // local a=Duel.GetAttacker()
    let a = duel::get_attacker(d).nil();
    // return a:IsPosition(POS_FACEUP_ATTACK) and a:IsRelateToBattle()
    card::is_position(d, a, POS_FACEUP_ATTACK) && card::is_relate_to_battle(d, a)
}
// s.posop(e,tp,eg,ep,ev,re,r,rp)
fn posop(d: &mut Duel, a: &EffArgs) {
    // local a=Duel.GetAttacker()
    let at = duel::get_attacker(d).nil();
    // if Duel.ChangePosition(a,POS_FACEUP_DEFENSE)~=0 then
    if duel::change_position(d, at, POS_FACEUP_DEFENSE as u8, None, None, None, false) != 0 {
        let h = effect::get_handler(d, a.e).nil();
        // e:GetHandler():CreateRelation(a,RESET_EVENT|RESETS_STANDARD)
        card::create_relation(d, h, at, RESET_EVENT | RESETS_STANDARD);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CANNOT_CHANGE_POSITION)
        effect::set_code(d, e1, EFFECT_CANNOT_CHANGE_POSITION);
        // e1:SetReset(RESETS_STANDARD_PHASE_END,3)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 3);
        // e1:SetCondition(s.poscon2)
        effect::set_condition(d, e1, Some(lua_fn(poscon2)));
        // a:RegisterEffect(e1)
        card::register_effect(d, at, e1, false);
    }
}
// s.poscon2(e) return e:GetOwner():IsRelateToCard(e:GetHandler()) end
fn poscon2(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let owner = effect::get_owner(d, e).nil();
    let h = effect::get_handler(d, e).nil();
    card::is_relate_to_card(d, owner, h)
}
