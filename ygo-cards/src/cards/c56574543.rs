//! Bujingi Sinyou
use super::prelude::*;

pub const ID: u32 = 56574543;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BUJIN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Increase ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.atkcon)
    effect::set_condition(d, e1, Some(condition(atkcon)));
    // e1:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e1, Some(cost::self_banish()));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.atkcon(e,tp,eg,ep,ev,re,r,rp)
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local phase=Duel.GetCurrentPhase()
    let phase = duel::get_current_phase(d);
    // if phase~=PHASE_DAMAGE or Duel.IsDamageCalculated() then return false end
    if phase != PHASE_DAMAGE || duel::is_damage_calculated(d) {
        return false;
    }
    // local c=Duel.GetAttackTarget()
    // if not c then return false end
    let Some(mut c) = duel::get_attack_target(d) else {
        return false;
    };
    // if c:IsControler(1-tp) then c=Duel.GetAttacker() end
    if card::is_controler(d, c, 1 - a.tp) {
        match duel::get_attacker(d) {
            Some(x) => c = x,
            None => return false,
        }
    }
    // return c and c:IsSetCard(SET_BUJIN) and c:IsRace(RACE_BEASTWARRIOR) and c:IsRelateToBattle()
    card::is_set_card(d, c, &[SET_BUJIN as u16]) && card::is_race(d, c, RACE_BEASTWARRIOR) && card::is_relate_to_battle(d, c)
}
// function s.atkop(e,tp,eg,ep,ev,re,r,rp,chk)
fn atkop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local a=Duel.GetAttacker()
    let mut atk = duel::get_attacker(d).nil();
    // local d=Duel.GetAttackTarget()
    let mut dfn = duel::get_attack_target(d).nil();
    // if not a:IsRelateToBattle() or a:IsFacedown() or not d:IsRelateToBattle() or d:IsFacedown() then return end
    if !card::is_relate_to_battle(d, atk) || card::is_facedown(d, atk) || !card::is_relate_to_battle(d, dfn) || card::is_facedown(d, dfn) {
        return;
    }
    // if a:GetControler()~=tp then a,d=d,a end
    if card::get_controler(d, atk) != tp {
        std::mem::swap(&mut atk, &mut dfn);
    }
    // if a:IsImmuneToEffect(e) then return end
    if card::is_immune_to_effect(d, atk, a.e) {
        return;
    }
    let h = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, h);
    // e1:SetOwnerPlayer(tp)
    effect::set_owner_player(d, e1, tp);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 1);
    // e1:SetValue(d:GetAttack())
    let dfn_atk = card::get_attack(d, dfn);
    effect::set_value(d, e1, dfn_atk as i64);
    // a:RegisterEffect(e1)
    card::register_effect(d, atk, e1, false);
    // local e2=Effect.CreateEffect(e:GetHandler())
    let e2 = effect::create_effect(d, h);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CHANGE_BATTLE_DAMAGE)
    effect::set_code(d, e2, EFFECT_CHANGE_BATTLE_DAMAGE);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetTargetRange(0,1)
    effect::set_target_range(d, e2, 0, 1);
    // e2:SetValue(HALF_DAMAGE)
    effect::set_value(d, e2, HALF_DAMAGE as i64);
    // e2:SetReset(RESET_PHASE|PHASE_DAMAGE)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_DAMAGE, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
}
