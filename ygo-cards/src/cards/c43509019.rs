//! Toon Defense
use super::prelude::*;

pub const ID: u32 = 43509019;

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
    // change battle target
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(s.cbcon)
    effect::set_condition(d, e2, Some(condition(cbcon)));
    // e2:SetTarget(s.cbtg)
    effect::set_target(d, e2, Some(cost_target(cbtg)));
    // e2:SetOperation(s.cbop)
    effect::set_operation(d, e2, Some(operation(cbop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cbcon(e,tp,eg,ep,ev,re,r,rp)`
fn cbcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local bt=Duel.GetAttackTarget()
    let bt = duel::get_attack_target(d);
    // return bt and bt:IsFaceup() and bt:IsLevelBelow(4) and bt:IsType(TYPE_TOON) and bt:GetControler()==e:GetHandlerPlayer()
    let Some(bt) = bt else { return false };
    card::is_faceup(d, bt)
        && card::is_level_below(d, bt, 4)
        && card::is_type(d, bt, TYPE_TOON)
        && card::get_controler(d, bt) == effect::get_handler_player(d, a.e)
}
/// `s.cbtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cbtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return not Duel.GetAttacker():IsHasEffect(EFFECT_CANNOT_DIRECT_ATTACK) end
    if a.chk == Some(0) {
        let at = duel::get_attacker(d).nil();
        return card::is_has_effect(d, at, EFFECT_CANNOT_DIRECT_ATTACK, None).is_empty();
    }
    true
}
/// `s.cbop(e,tp,eg,ep,ev,re,r,rp)`
fn cbop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // Duel.ChangeAttackTarget(nil)
    duel::change_attack_target(d, None, false);
}
