//! Doble Passe
use super::prelude::*;

pub const ID: u32 = 79997591;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // change battle target
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetCondition(s.cbcon)
    effect::set_condition(d, e1, Some(condition(cbcon)));
    // e1:SetTarget(s.cbtg)
    effect::set_target(d, e1, Some(cost_target(cbtg)));
    // e1:SetOperation(s.cbop)
    effect::set_operation(d, e1, Some(operation(cbop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `function(e,tp,eg,ep,ev,re,r,rp)`
fn cbcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local bt=Duel.GetAttackTarget()
    // return bt and bt:IsLocation(LOCATION_MZONE) and bt:IsPosition(POS_FACEUP_ATTACK) and bt:IsControler(tp)
    match duel::get_attack_target(d) {
        Some(bt) => {
            card::is_location(d, bt, LOCATION_MZONE)
                && card::is_position(d, bt, POS_FACEUP_ATTACK)
                && card::is_controler(d, bt, a.tp)
        }
        None => false,
    }
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cbtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // return not Duel.GetAttacker():IsHasEffect(EFFECT_CANNOT_DIRECT_ATTACK)
        let at = duel::get_attacker(d).nil();
        return card::is_has_effect(d, at, EFFECT_CANNOT_DIRECT_ATTACK, None).is_empty();
    }
    true
}
/// `function(e,tp,eg,ep,ev,re,r,rp)`
fn cbop(d: &mut Duel, a: &EffArgs) {
    // local at=Duel.GetAttacker()
    let at = duel::get_attacker(d);
    // local bt=Duel.GetAttackTarget()
    let bt = duel::get_attack_target(d).nil();
    // if not (bt:IsRelateToBattle() and bt:IsControler(tp)) then return end
    if !(card::is_relate_to_battle(d, bt) && card::is_controler(d, bt, a.tp)) {
        return;
    }
    // if at:CanAttack() and not at:IsStatus(STATUS_ATTACK_CANCELED) and Duel.Damage(1-tp,bt:GetAttack(),REASON_EFFECT)>0 then
    let at = at.nil();
    if card::can_attack(d, at) && !card::is_status(d, at, STATUS_ATTACK_CANCELED) {
        let atk = card::get_attack(d, bt) as i64;
        if duel::damage(d, 1 - a.tp, atk, REASON_EFFECT, false, None) > 0 {
            // Duel.ChangeAttackTarget(nil)
            duel::change_attack_target(d, None, false);
        }
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
    // e1:SetReset(RESETS_STANDARD_PHASE_END|RESET_SELF_TURN)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END | RESET_SELF_TURN, 1);
    // bt:RegisterEffect(e1)
    card::register_effect(d, bt, e1, false);
}
