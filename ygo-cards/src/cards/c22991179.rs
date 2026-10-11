//! Insect Neglect
use super::prelude::*;

pub const ID: u32 = 22991179;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Negate the attack of an opponent's monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCondition(function(e,tp) return Duel.GetAttacker():IsControler(1-tp) end)
    effect::set_condition(d, e1, Some(lua_fn(attack_cond)));
    // e1:SetCost(s.negatkcost)
    effect::set_cost(d, e1, Some(cost_target(negatkcost)));
    // e1:SetTarget(s.negatktg)
    effect::set_target(d, e1, Some(cost_target(negatktg)));
    // e1:SetOperation(function() Duel.NegateAttack() end)
    effect::set_operation(d, e1, Some(lua_fn(negate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `function(e,tp) return Duel.GetAttacker():IsControler(1-tp) end`
fn attack_cond(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    let att = duel::get_attacker(d).nil();
    card::is_controler(d, att, 1 - tp)
}
/// `s.negatkcostfilter(c)`
fn negatkcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_INSECT) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    card::is_race(d, c, RACE_INSECT) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, true, false)
}
/// `s.negatkcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negatkcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.negatkcostfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&negatkcostfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.negatkcostfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&negatkcostfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.negatktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negatktg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // local ac=Duel.GetAttacker()
        let ac = duel::get_attacker(d).nil();
        // return ac:IsOnField() and ac:IsRelateToBattle()
        return card::is_on_field(d, ac) && card::is_relate_to_battle(d, ac);
    }
    true
}
/// `function() Duel.NegateAttack() end`
fn negate(d: &mut Duel, _a: Args) {
    duel::negate_attack(d);
}
