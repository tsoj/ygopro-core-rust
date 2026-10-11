//! Barrier Wave
use super::prelude::*;

pub const ID: u32 = 96700602;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_POSITION+CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_POSITION + CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `function s.condition(e,tp,eg,ep,ev,re,r,rp) return Duel.IsTurnPlayer(1-tp) end`
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp)
}
/// `s.cost`: `Duel.CheckReleaseGroupCost(tp,Card.IsType,1,false,nil,nil,TYPE_SYNCHRO)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&|d, c| card::is_type(d, c, TYPE_SYNCHRO)), 1, 1, false, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,Card.IsType,1,1,false,nil,nil,TYPE_SYNCHRO)
    let g = duel::select_release_group_cost(d, a.tp, Some(&|d, c| card::is_type(d, c, TYPE_SYNCHRO)), 1, 1, false, None, NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.filter(c)`: `c:IsAttackPos() and c:IsCanChangePosition()`
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_attack_pos(d, c) && card::is_can_change_position(d, c)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // local tc=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let tc = group::get_first(d, &eg).nil();
    // if tc:IsLocation(LOCATION_MZONE) then
    if card::is_location(d, tc, LOCATION_MZONE) {
        // Duel.SetTargetCard(tc)
        duel::set_target_card(d, tc);
        // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,tc:GetDefense())
        let def = card::get_defense(d, tc);
        duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, def);
    }
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // local g=Duel.GetMatchingGroup(s.filter,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&filter), a.tp, 0, LOCATION_MZONE, NO_EX);
    // if Duel.ChangePosition(g,POS_FACEUP_DEFENSE,POS_FACEDOWN_DEFENSE,0,0)~=0 then
    if duel::change_position(d, g, POS_FACEUP_DEFENSE as u8, Some(POS_FACEDOWN_DEFENSE as u8), Some(0), Some(0), false) != 0 {
        // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
        let tc = tc.nil();
        if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Damage(1-tp,tc:GetDefense(),REASON_EFFECT)
            let def = card::get_defense(d, tc);
            duel::damage(d, 1 - a.tp, def as i64, REASON_EFFECT, false, None);
        }
    }
}
