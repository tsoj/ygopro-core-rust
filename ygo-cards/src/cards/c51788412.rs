//! Chaos Ancient Gear Giant
use super::prelude::*;

pub const ID: u32 = 51788412;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ANCIENT_GEAR], material_setcode: &[SET_ANCIENT_GEAR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMixN(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_ANCIENT_GEAR),4)
    fusion::add_proc_mix_n(d, c, true, true, vec![(fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_ANCIENT_GEAR as u16], Some(fc), st, Some(tp)))), 4)]);
    // special summon condition
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(aux.fuslimit)
    effect::set_value_fn(d, e1, aux::fuslimit());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // immune
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e2, EFFECT_IMMUNE_EFFECT);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(s.efilter)
    effect::set_value_fn(d, e2, lua_fn(efilter));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // activate limit
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e3, EFFECT_CANNOT_ACTIVATE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(0,1)
    effect::set_target_range(d, e3, 0, 1);
    // e3:SetCondition(s.actcon)
    effect::set_condition(d, e3, Some(condition(actcon)));
    // e3:SetValue(s.actlimit)
    effect::set_value_fn(d, e3, lua_fn(actlimit));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // attack all
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetCode(EFFECT_ATTACK_ALL)
    effect::set_code(d, e4, EFFECT_ATTACK_ALL);
    // e4:SetValue(1)
    effect::set_value(d, e4, 1);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // pierce
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    // e5:SetCode(EFFECT_PIERCE)
    effect::set_code(d, e5, EFFECT_PIERCE);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
// function s.efilter(e,te)
fn efilter(d: &mut Duel, a: Args) -> bool {
    // return te:IsSpellTrapEffect()
    effect::is_spell_trap_effect(d, a.e(1))
}
// function s.actcon(e)
fn actcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // local ph=Duel.GetCurrentPhase()
    let _ph = duel::get_current_phase(d);
    // return Duel.IsBattlePhase()
    duel::is_battle_phase(d, None)
}
// function s.actlimit(e,re,tp)
fn actlimit(d: &mut Duel, a: Args) -> bool {
    // return re:IsMonsterEffect()
    effect::is_monster_effect(d, a.e(1))
}
