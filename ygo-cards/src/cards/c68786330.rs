//! Attack Pheromones
use super::prelude::*;

pub const ID: u32 = 68786330;

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
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_POSITION)
    effect::set_category(d, e2, CATEGORY_POSITION);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e2, EVENT_DAMAGE_STEP_END);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(s.poscon)
    effect::set_condition(d, e2, Some(condition(poscon)));
    // e2:SetOperation(s.posop)
    effect::set_operation(d, e2, Some(operation(posop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.poscon(e,tp,eg,ep,ev,re,r,rp)`
fn poscon(d: &mut Duel, _a: &EffArgs) -> bool {
    // local a=Duel.GetAttacker()
    let a = duel::get_attacker(d);
    // local d=Duel.GetAttackTarget()
    let t = duel::get_attack_target(d);
    // return a and d and a:IsRelateToBattle() and d:IsRelateToBattle() and a:IsRace(RACE_REPTILE) and d:IsDefensePos()
    match (a, t) {
        (Some(a), Some(t)) => card::is_relate_to_battle(d, a) && card::is_relate_to_battle(d, t) && card::is_race(d, a, RACE_REPTILE) && card::is_defense_pos(d, t),
        _ => false,
    }
}
/// `s.posop(e,tp,eg,ep,ev,re,r,rp)`
fn posop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local a=Duel.GetAttackTarget()
    let t = duel::get_attack_target(d).nil();
    // Duel.ChangePosition(a,POS_FACEUP_ATTACK)
    duel::change_position(d, t, POS_FACEUP_ATTACK as u8, None, None, None, false);
}
