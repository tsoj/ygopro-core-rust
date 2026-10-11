//! Dark City
use super::prelude::*;

pub const ID: u32 = 53527835;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_DESTINY_HERO],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // atk up
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(condition(atkcon)));
    // e2:SetTarget(s.atktg)
    effect::set_target(d, e2, Some(effect_card_fn(atktg)));
    // e2:SetValue(s.atkval)
    effect::set_value_fn(d, e2, effect_card_value(atkval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.atkcon(e)`
fn atkcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // s[0]=false
    set_s_var(d, ID, 0, Value::Bool(false));
    // return Duel.IsPhase(PHASE_DAMAGE_CAL) and Duel.GetAttackTarget()
    duel::is_phase(d, PHASE_DAMAGE_CAL) && duel::get_attack_target(d).is_some()
}
/// `s.atktg(e,c)`
fn atktg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c==Duel.GetAttacker() and c:IsSetCard(SET_DESTINY_HERO)
    duel::get_attacker(d) == Some(c) && card::is_set_card(d, c, &[SET_DESTINY_HERO as u16])
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // local d=Duel.GetAttackTarget()
    let tgt = duel::get_attack_target(d);
    // if s[0] or c:GetAttack()<d:GetAttack() then
    if s_var(d, ID, 0).truthy() || card::get_attack(d, c) < card::get_attack(d, tgt.nil()) {
        // s[0]=true
        set_s_var(d, ID, 0, Value::Bool(true));
        // return 1000
        1000
    } else {
        // return 0
        0
    }
}
