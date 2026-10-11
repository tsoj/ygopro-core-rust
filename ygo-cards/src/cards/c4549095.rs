//! Battlin' Boxer Counterpunch
use super::prelude::*;

pub const ID: u32 = 4549095;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BATTLIN_BOXER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atkup
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    // e1:SetRange(LOCATION_HAND|LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_GRAVE);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e1, Some(cost::self_banish()));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local phase=Duel.GetCurrentPhase()
    let phase = duel::get_current_phase(d);
    // if phase~=PHASE_DAMAGE or Duel.IsDamageCalculated() then return false end
    if phase != PHASE_DAMAGE || duel::is_damage_calculated(d) {
        return false;
    }
    // local a=Duel.GetAttacker()
    let at = duel::get_attacker(d).nil();
    // local d=Duel.GetAttackTarget()
    let dt = duel::get_attack_target(d);
    // return (a:IsControler(tp) and a:IsSetCard(SET_BATTLIN_BOXER) and a:IsRelateToBattle())
    //     or (d and d:IsControler(tp) and d:IsSetCard(SET_BATTLIN_BOXER) and d:IsRelateToBattle())
    if card::is_controler(d, at, a.tp) && card::is_set_card(d, at, &[SET_BATTLIN_BOXER as u16]) && card::is_relate_to_battle(d, at) {
        return true;
    }
    match dt {
        Some(dt) => {
            card::is_controler(d, dt, a.tp) && card::is_set_card(d, dt, &[SET_BATTLIN_BOXER as u16]) && card::is_relate_to_battle(d, dt)
        }
        None => false,
    }
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp,chk)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local a=Duel.GetAttacker()
    let mut at = duel::get_attacker(d);
    // if Duel.IsTurnPlayer(1-tp) then a=Duel.GetAttackTarget() end
    if duel::is_turn_player(d, 1 - a.tp) {
        at = duel::get_attack_target(d);
    }
    // if not a:IsRelateToBattle() then return end
    let at = at.nil();
    if !card::is_relate_to_battle(d, at) {
        return;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // e1:SetValue(1000)
    effect::set_value(d, e1, 1000);
    // a:RegisterEffect(e1)
    card::register_effect(d, at, e1, false);
}
