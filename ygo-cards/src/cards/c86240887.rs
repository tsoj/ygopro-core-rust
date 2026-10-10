//! Buster Blader, the Dragon Destroyer Swordsman
use super::prelude::*;

pub const ID: u32 = 86240887;

pub const INFO: ScriptInfo = ScriptInfo { material_setcode: &[SET_BUSTER_BLADER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,CARD_BUSTER_BLADER,aux.FilterBoolFunctionEx(Card.IsRace,RACE_DRAGON))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::Code(CARD_BUSTER_BLADER),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_race_ctx(d, c, RACE_DRAGON, Some(fc), st, Some(tp)))),
        ],
    );
    // cannot direct attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CANNOT_DIRECT_ATTACK)
    effect::set_code(d, e1, EFFECT_CANNOT_DIRECT_ATTACK);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // ATK Gain
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(s.val)
    effect::set_value_fn(d, e2, effect_card_value(val));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // spsummon condition
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e4, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e4:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e4, EFFECT_SPSUMMON_CONDITION);
    // e4:SetValue(aux.fuslimit)
    effect::set_value_fn(d, e4, aux::fuslimit());
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Pos Change
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD);
    // e5:SetCode(EFFECT_SET_POSITION)
    effect::set_code(d, e5, EFFECT_SET_POSITION);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetTarget(s.target)
    effect::set_target(d, e5, Some(effect_card_fn(target)));
    // e5:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e5, 0, LOCATION_MZONE);
    // e5:SetValue(POS_FACEUP_DEFENSE)
    effect::set_value(d, e5, POS_FACEUP_DEFENSE as i64);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // Prevent Activation
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e6, EFFECT_TYPE_FIELD);
    // e6:SetRange(LOCATION_MZONE)
    effect::set_range(d, e6, LOCATION_MZONE);
    // e6:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e6, EFFECT_CANNOT_ACTIVATE);
    // e6:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e6, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e6:SetTargetRange(0,1)
    effect::set_target_range(d, e6, 0, 1);
    // e6:SetValue(s.aclimit)
    effect::set_value_fn(d, e6, lua_fn(aclimit));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
    // pierce
    // local e7=Effect.CreateEffect(c)
    let e7 = effect::create_effect(d, c);
    // e7:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e7, EFFECT_TYPE_SINGLE);
    // e7:SetCode(EFFECT_PIERCE)
    effect::set_code(d, e7, EFFECT_PIERCE);
    // c:RegisterEffect(e7)
    card::register_effect(d, c, e7, false);
}
/// `s.val(e,c)`
fn val(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return Duel.GetMatchingGroupCount(s.filter,c:GetControler(),0,LOCATION_GRAVE|LOCATION_MZONE,nil)*1000
    let p = card::get_controler(d, c);
    duel::get_matching_group_count(d, Some(&filter), p, 0, LOCATION_GRAVE | LOCATION_MZONE, NO_EX) as i64 * 1000
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_DRAGON) and (c:IsLocation(LOCATION_GRAVE) or c:IsFaceup())
    card::is_race(d, c, RACE_DRAGON) && (card::is_location(d, c, LOCATION_GRAVE) || card::is_faceup(d, c))
}
/// `s.target(e,c)`
fn target(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsRace(RACE_DRAGON)
    card::is_race(d, c, RACE_DRAGON)
}
/// `s.aclimit(e,re,tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    // return re:GetHandler():IsRace(RACE_DRAGON) and re:IsMonsterEffect()
    let h = effect::get_handler(d, re).nil();
    card::is_race(d, h, RACE_DRAGON) && effect::is_monster_effect(d, re)
}
