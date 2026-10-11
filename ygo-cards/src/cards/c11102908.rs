//! Shien's Castle of Mist
use super::prelude::*;

pub const ID: u32 = 11102908;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SIX_SAMURAI], ..ScriptInfo::DEFAULT };

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
    // atk down
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
    // e2:SetValue(-500)
    effect::set_value(d, e2, -500);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.atkcon(e)`
fn atkcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // local d=Duel.GetAttackTarget()
    let at = duel::get_attack_target(d);
    // return Duel.IsPhase(PHASE_DAMAGE_CAL) and d and d:IsSetCard(SET_SIX_SAMURAI)
    duel::is_phase(d, PHASE_DAMAGE_CAL)
        && match at {
            Some(at) => card::is_set_card(d, at, &[SET_SIX_SAMURAI as u16]),
            None => false,
        }
}
/// `s.atktg(e,c)`
fn atktg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c==Duel.GetAttacker()
    duel::get_attacker(d) == Some(c)
}
