//! Max Warrior
use super::prelude::*;

pub const ID: u32 = 94538053;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetCondition(s.condtion)
    effect::set_condition(d, e1, Some(condition(condtion)));
    // e1:SetValue(400)
    effect::set_value(d, e1, 400);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // addown
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    // e2:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(condition(atkcon)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn condtion(d: &mut Duel, a: &EffArgs) -> bool {
    // local ph=Duel.GetCurrentPhase()
    let ph = duel::get_current_phase(d);
    // return (ph==PHASE_DAMAGE or ph==PHASE_DAMAGE_CAL) and Duel.GetAttacker()==e:GetHandler() and Duel.GetAttackTarget()~=nil
    (ph == PHASE_DAMAGE || ph == PHASE_DAMAGE_CAL)
        && duel::get_attacker(d) == Some(effect::get_handler(d, a.e).nil())
        && duel::get_attack_target(d).is_some()
}
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsRelateToBattle() and e:GetHandler():IsFaceup()
    let h = effect::get_handler(d, a.e).nil();
    card::is_relate_to_battle(d, h) && card::is_faceup(d, h)
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToBattle() and c:IsFaceup() then
    if card::is_relate_to_battle(d, c) && card::is_faceup(d, c) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_BASE_ATTACK)
        effect::set_code(d, e1, EFFECT_SET_BASE_ATTACK);
        // e1:SetValue(c:GetBaseAttack()/2)
        let base_atk = card::get_base_attack(d, c);
        effect::set_value(d, e1, lua_int(base_atk as f64 / 2.0));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE|RESET_PHASE|PHASE_STANDBY|RESET_SELF_TURN)
        effect::set_reset(
            d,
            e1,
            RESET_EVENT | RESETS_STANDARD_DISABLE | RESET_PHASE | PHASE_STANDBY | RESET_SELF_TURN,
            1,
        );
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_SET_BASE_DEFENSE)
        effect::set_code(d, e2, EFFECT_SET_BASE_DEFENSE);
        // e2:SetValue(c:GetBaseDefense()/2)
        let base_def = card::get_base_defense(d, c);
        effect::set_value(d, e2, lua_int(base_def as f64 / 2.0));
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
        // local e3=e1:Clone()
        let e3 = effect::clone(d, e1);
        // e3:SetCode(EFFECT_CHANGE_LEVEL)
        effect::set_code(d, e3, EFFECT_CHANGE_LEVEL);
        // e3:SetValue(2)
        effect::set_value(d, e3, 2);
        // c:RegisterEffect(e3)
        card::register_effect(d, c, e3, false);
    }
}
