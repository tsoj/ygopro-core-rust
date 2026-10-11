//! Otherworld - The "A" Zone
use super::prelude::*;

pub const ID: u32 = 60946968;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ALIEN],
    ..ScriptInfo::DEFAULT
};

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
    // atk def
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    // e2:SetCondition(s.adcon)
    effect::set_condition(d, e2, Some(lua_fn(|d, a| adcon(d, a.e(0)))));
    // e2:SetTarget(s.adtg)
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| adtg(d, c))));
    // e2:SetValue(-300)
    effect::set_value(d, e2, -300);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.adcon(e)`
fn adcon(d: &mut Duel, e: EffectId) -> bool {
    // if Duel.GetCurrentPhase()~=PHASE_DAMAGE_CAL then return false end
    if duel::get_current_phase(d) != PHASE_DAMAGE_CAL {
        return false;
    }
    // local d=Duel.GetAttackTarget()
    // if not d then return false end
    let Some(mut dc) = duel::get_attack_target(d) else {
        return false;
    };
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, e);
    // if d:IsControler(1-tp) then d=Duel.GetAttacker() end
    if card::is_controler(d, dc, 1 - tp) {
        dc = duel::get_attacker(d).nil();
    }
    // return d:IsSetCard(SET_ALIEN)
    card::is_set_card(d, dc, &[SET_ALIEN as u16])
}
/// `s.adtg(e,c)`
fn adtg(d: &mut Duel, c: CardId) -> bool {
    // return c==Duel.GetAttacker() or c==Duel.GetAttackTarget()
    Some(c) == duel::get_attacker(d) || Some(c) == duel::get_attack_target(d)
}
