//! Steel Cavalry of Dinon
use super::prelude::*;

pub const ID: u32 = 2396042;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // --atk/def
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_BATTLE_START)
    effect::set_code(d, e1, EVENT_BATTLE_START);
    // e1:SetCondition(s.adcon)
    effect::set_condition(d, e1, Some(condition(adcon)));
    // e1:SetOperation(s.adop)
    effect::set_operation(d, e1, Some(operation(adop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// s.adcon(e,tp,eg,ep,ev,re,r,rp)
fn adcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // return bc and bc:IsFaceup() and bc:IsType(TYPE_PENDULUM)
    match bc {
        Some(bc) => card::is_faceup(d, bc) && card::is_type(d, bc, TYPE_PENDULUM),
        None => false,
    }
}
// s.adop(e,tp,eg,ep,ev,re,r,rp)
fn adop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_DEFENSE_FINAL)
        effect::set_code(d, e1, EFFECT_SET_DEFENSE_FINAL);
        // e1:SetValue(c:GetDefense()/2)
        let def = card::get_defense(d, c);
        effect::set_value(d, e1, lua_int(def as f64 / 2.0));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE|RESET_PHASE|PHASE_DAMAGE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE | RESET_PHASE | PHASE_DAMAGE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e2, EFFECT_SET_ATTACK_FINAL);
        // e2:SetValue(c:GetAttack()/2)
        let atk = card::get_attack(d, c);
        effect::set_value(d, e2, lua_int(atk as f64 / 2.0));
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
    }
}
