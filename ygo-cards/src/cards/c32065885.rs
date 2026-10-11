//! Heartfelt Appeal
use super::prelude::*;

pub const ID: u32 = 32065885;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e1, CATEGORY_CONTROL);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e1, EVENT_BATTLE_DAMAGE);
    // e1:SetCondition(s.ctlcon)
    effect::set_condition(d, e1, Some(condition(ctlcon)));
    // e1:SetTarget(s.ctltg)
    effect::set_target(d, e1, Some(cost_target(ctltg)));
    // e1:SetOperation(s.ctlop)
    effect::set_operation(d, e1, Some(operation(ctlop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.ctlcon(e,tp,eg,ep,ev,re,r,rp)`
fn ctlcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==tp and Duel.GetAttackTarget()==nil and Duel.GetAttacker():IsControler(1-tp)
    if a.ep != a.tp || duel::get_attack_target(d).is_some() {
        return false;
    }
    let atk = duel::get_attacker(d).nil();
    card::is_controler(d, atk, 1 - a.tp)
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsControlerCanBeChanged() and c:IsDefenseAbove(0)
    card::is_faceup(d, c) && card::is_controler_can_be_changed(d, c, false, None) && card::is_defense_above(d, c, 0)
}
/// `s.ctltg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn ctltg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,nil,1,1-tp,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, None, 1, 1 - a.tp, LOCATION_MZONE as i32);
    true
}
/// `s.filter1(c)`
fn filter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsDefenseAbove(0)
    card::is_faceup(d, c) && card::is_defense_above(d, c, 0)
}
/// `s.ctlop(e,tp,eg,ep,ev,re,r,rp)`
fn ctlop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.filter1,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&filter1), a.tp, 0, LOCATION_MZONE, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local sg=g:GetMaxGroup(Card.GetDefense)
    let mut sg = group::get_max_group(d, &g, &|d, c| card::get_defense(d, c) as i64).map(|r| r.0).unwrap_or_default();
    // if #sg>1 then
    if sg.len() > 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONTROL as u64);
        // sg=sg:Select(tp,1,1,nil)
        sg = group::select(d, &sg, a.tp, 1, 1, false, NO_EX).nil();
        // Duel.HintSelection(sg)
        duel::hint_selection(d, sg.clone(), true);
    }
    // local tc=sg:GetFirst()
    let tc = group::get_first(d, &sg).nil();
    // if Duel.GetControl(tc,tp,PHASE_END,2)~=0 then
    if duel::get_control(d, tc, a.tp, PHASE_END as u16, 2, None, None) != 0 {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e1, EFFECT_DISABLE);
        // e1:SetReset(RESET_EVENT|(RESETS_STANDARD_PHASE_END&~RESET_TURN_SET),2)
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD_PHASE_END & !RESET_TURN_SET), 2);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_DISABLE_EFFECT)
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        // e2:SetReset(RESET_EVENT|(RESETS_STANDARD_PHASE_END&~RESET_TURN_SET),2)
        effect::set_reset(d, e2, RESET_EVENT | (RESETS_STANDARD_PHASE_END & !RESET_TURN_SET), 2);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
        // e3:SetCode(EFFECT_CANNOT_ATTACK)
        effect::set_code(d, e3, EFFECT_CANNOT_ATTACK);
        // e3:SetReset(RESET_EVENT|(RESETS_STANDARD_PHASE_END&~RESET_TURN_SET),2)
        effect::set_reset(d, e3, RESET_EVENT | (RESETS_STANDARD_PHASE_END & !RESET_TURN_SET), 2);
        // tc:RegisterEffect(e3)
        card::register_effect(d, tc, e3, false);
    }
}
