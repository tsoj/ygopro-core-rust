//! Destruction Punch
use super::prelude::*;

pub const ID: u32 = 5616412;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e1, EVENT_DAMAGE_STEP_END);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local a=Duel.GetAttacker()
    // local d=Duel.GetAttackTarget()
    // return d and a:IsControler(1-tp) and a:IsRelateToBattle() and d:IsDefensePos() and d:IsRelateToBattle() and d:GetDefense()>a:GetAttack()
    let Some(dt) = duel::get_attack_target(d) else {
        return false;
    };
    let at = duel::get_attacker(d).nil();
    card::is_controler(d, at, 1 - a.tp)
        && card::is_relate_to_battle(d, at)
        && card::is_defense_pos(d, dt)
        && card::is_relate_to_battle(d, dt)
        && card::get_defense(d, dt) > card::get_attack(d, at)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetCard(Duel.GetAttacker())
    let at = duel::get_attacker(d).nil();
    duel::set_target_card(d, at);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,Duel.GetAttacker(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(at)), 1, 0, 0);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
