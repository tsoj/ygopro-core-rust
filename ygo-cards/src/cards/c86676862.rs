//! Evil HERO Malicious Fiend
use super::prelude::*;

pub const ID: u32 = 86676862;

pub const INFO: ScriptInfo = ScriptInfo {
    material_setcode: &[SET_HERO, SET_EVIL_HERO],
    dark_calling: true,
    listed_names: &[CARD_DARK_FUSION, 58554959],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion material
    // Fusion.AddProcMix(c,true,true,58554959,s.ffilter)
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![fusion::Mat::Code(58554959), fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| ffilter(d, c, fc, st, tp)))],
    );
    // lizard check
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetCode(CARD_CLOCK_LIZARD)
    effect::set_code(d, e0, CARD_CLOCK_LIZARD);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCondition(s.lizcon)
    effect::set_condition(d, e0, Some(condition(lizcon)));
    // e0:SetValue(1)
    effect::set_value(d, e0, 1);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Special Summon condition
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(aux.EvilHeroLimit)
    effect::set_value_fn(d, e1, aux::evil_hero_limit());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Change all monsters to Attack position
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_SET_POSITION)
    effect::set_code(d, e2, EFFECT_SET_POSITION);
    // e2:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e2, EFFECT_FLAG_SET_AVAILABLE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    // e2:SetCondition(s.poscon)
    effect::set_condition(d, e2, Some(condition(poscon)));
    // e2:SetValue(POS_FACEUP_ATTACK)
    effect::set_value(d, e2, POS_FACEUP_ATTACK as i64);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Monsters must attack
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_MUST_ATTACK)
    effect::set_code(d, e3, EFFECT_MUST_ATTACK);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e3, 0, LOCATION_MZONE);
    // e3:SetCondition(s.poscon)
    effect::set_condition(d, e3, Some(condition(poscon)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EFFECT_MUST_ATTACK_MONSTER)
    effect::set_code(d, e4, EFFECT_MUST_ATTACK_MONSTER);
    // e4:SetValue(s.atklimit)
    effect::set_value_fn(d, e4, lua_fn(atklimit));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.ffilter(c,fc,sumtype,tp)`
fn ffilter(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsRace(RACE_FIEND,fc,sumtype,tp) and c:GetLevel()>=6
    card::is_race_ctx(d, c, RACE_FIEND, Some(fc), sumtype, Some(tp)) && card::get_level(d, c) >= 6
}
/// `s.lizcon(e,tp,eg,ep,ev,re,r,rp)`
fn lizcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let _c = effect::get_handler(d, a.e);
    // return not Duel.IsPlayerAffectedByEffect(e:GetHandlerPlayer(),EFFECT_SUPREME_CASTLE)
    let hp = effect::get_handler_player(d, a.e);
    duel::is_player_affected_by_effect(d, hp, EFFECT_SUPREME_CASTLE).is_none()
}
/// `s.poscon(e)`
fn poscon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetTurnPlayer()==1-e:GetHandlerPlayer() and Duel.IsBattlePhase()
    let hp = effect::get_handler_player(d, a.e);
    duel::get_turn_player(d) == 1 - hp && duel::is_battle_phase(d, None)
}
/// `s.atklimit(e,c)`: `return c==e:GetHandler()`
fn atklimit(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    a.card(1) == effect::get_handler(d, e)
}
