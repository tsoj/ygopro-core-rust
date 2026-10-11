//! Blackwing - Jetstream the Blue Sky
use super::prelude::*;

pub const ID: u32 = 87390067;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BLACKWING], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // indes
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e2, EVENT_PRE_DAMAGE_CALCULATE);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetCost(Cost.SelfToGrave)
    effect::set_cost(d, e2, Some(cost::self_to_grave()));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local a=Duel.GetAttacker()
    let att = duel::get_attacker(d);
    // local d=Duel.GetAttackTarget()
    let dd = duel::get_attack_target(d);
    // if not d then return false end
    let Some(dd) = dd else {
        return false;
    };
    // return (a:IsControler(tp) and a:IsSetCard(SET_BLACKWING)) or (d:IsControler(tp) and d:IsSetCard(SET_BLACKWING))
    let att = att.nil();
    (card::is_controler(d, att, a.tp) && card::is_set_card(d, att, &[SET_BLACKWING as u16]))
        || (card::is_controler(d, dd, a.tp) && card::is_set_card(d, dd, &[SET_BLACKWING as u16]))
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp,chk)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local a=Duel.GetAttacker()
    let mut att = duel::get_attacker(d);
    // if Duel.IsTurnPlayer(1-tp) then a=Duel.GetAttackTarget() end
    if duel::is_turn_player(d, 1 - a.tp) {
        att = duel::get_attack_target(d);
    }
    // if not a:IsRelateToBattle() then return end
    let att = att.nil();
    if !card::is_relate_to_battle(d, att) {
        return;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE_CAL)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE_CAL, 1);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // a:RegisterEffect(e1)
    card::register_effect(d, att, e1, false);
}
