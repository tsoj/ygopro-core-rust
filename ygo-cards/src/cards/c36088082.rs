//! Bull Blader
use super::prelude::*;

pub const ID: u32 = 36088082;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // level change
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.regcon)
    effect::set_condition(d, e1, Some(condition(regcon)));
    // e1:SetOperation(s.regop)
    effect::set_operation(d, e1, Some(operation(regop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.regcon(e,tp,eg,ep,ev,re,r,rp)`
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return (e:GetHandler()==Duel.GetAttacker() and Duel.GetAttackTarget()~=nil) or e:GetHandler()==Duel.GetAttackTarget()
    let h = effect::get_handler(d, a.e);
    let attacker = duel::get_attacker(d);
    let at = duel::get_attack_target(d);
    (h == attacker && at.is_some()) || h == at
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_NO_BATTLE_DAMAGE)
        effect::set_code(d, e1, EFFECT_NO_BATTLE_DAMAGE);
        // e1:SetReset(RESET_PHASE|PHASE_DAMAGE)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_AVOID_BATTLE_DAMAGE)
        effect::set_code(d, e2, EFFECT_AVOID_BATTLE_DAMAGE);
        // e2:SetValue(1)
        effect::set_value(d, e2, 1);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e3:SetCode(EVENT_BATTLED)
        effect::set_code(d, e3, EVENT_BATTLED);
        // e3:SetOperation(s.desop)
        effect::set_operation(d, e3, Some(operation(desop)));
        // e3:SetReset(RESET_PHASE|PHASE_DAMAGE)
        effect::set_reset(d, e3, RESET_PHASE | PHASE_DAMAGE, 1);
        // Duel.RegisterEffect(e3,tp)
        duel::register_effect(d, e3, tp);
    }
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tg=e:GetOwner():GetBattleTarget()
    let owner = effect::get_owner(d, a.e).nil();
    let tg = card::get_battle_target(d, owner);
    // if tg then
    if let Some(tg) = tg {
        // Duel.Destroy(tg,REASON_EFFECT)
        duel::destroy(d, tg, REASON_EFFECT, None, None);
    }
}
