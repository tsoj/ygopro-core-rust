//! Mirror Gate
use super::prelude::*;

pub const ID: u32 = 43452193;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ELEMENTAL_HERO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e1, CATEGORY_CONTROL);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local at=Duel.GetAttackTarget()
    let at = duel::get_attack_target(d);
    // return Duel.IsTurnPlayer(1-tp) and at and at:IsFaceup() and at:IsSetCard(SET_ELEMENTAL_HERO)
    if !duel::is_turn_player(d, 1 - a.tp) {
        return false;
    }
    match at {
        Some(at) => card::is_faceup(d, at) && card::is_set_card(d, at, &[SET_ELEMENTAL_HERO as u16]),
        None => false,
    }
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local a=Duel.GetAttacker()
    let atk = duel::get_attacker(d).nil();
    // local at=Duel.GetAttackTarget()
    let at = duel::get_attack_target(d).nil();
    // if chk==0 then return a:IsOnField() and a:IsAbleToChangeControler()
    //     and at:IsOnField() and at:IsAbleToChangeControler() end
    if a.chk == Some(0) {
        return card::is_on_field(d, atk)
            && card::is_able_to_change_controler(d, atk)
            && card::is_on_field(d, at)
            && card::is_able_to_change_controler(d, at);
    }
    // local g=Group.FromCards(a,at)
    let mut g = CardSet::from_card(atk);
    group::add_card(&mut g, &CardSet::from_card(at));
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g.clone());
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,g,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(g), 2, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local a=Duel.GetAttacker()
    let atk = duel::get_attacker(d).nil();
    // local at=Duel.GetAttackTarget()
    let at = duel::get_attack_target(d);
    // if a:IsRelateToEffect(e) and a:CanAttack() and at:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, atk, a.e) && card::can_attack(d, atk) {
        let at = at.nil();
        if card::is_relate_to_effect(d, at, a.e) {
            // if Duel.SwapControl(a,at,RESET_PHASE|PHASE_END,1) then
            if duel::swap_control(d, atk, at, (RESET_PHASE | PHASE_END) as u16, 1) {
                // Duel.CalculateDamage(a,at)
                duel::calculate_damage(d, atk, Some(at), false);
            }
        }
    }
}
