//! Clear Vicious Knight
use super::prelude::*;

pub const ID: u32 = 70095046;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_CLEAR_WORLD],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddNormalSummonProcedure(c,true,true,1,1,SUMMON_TYPE_TRIBUTE,aux.Stringid(id,0),function(c) return c:ListsCode(CARD_CLEAR_WORLD) end)
    aux::add_normal_summon_procedure(d, c, true, true, Some(1), Some(1), Some(SUMMON_TYPE_TRIBUTE), Some(aux::stringid(ID, 0)), Some(std::sync::Arc::new(tribute_filter)), None);
    // aux.AddNormalSetProcedure(c,true,true,1,1,SUMMON_TYPE_TRIBUTE,aux.Stringid(id,0),function(c) return c:ListsCode(CARD_CLEAR_WORLD) end)
    aux::add_normal_set_procedure(d, c, true, true, Some(1), Some(1), Some(SUMMON_TYPE_TRIBUTE), Some(aux::stringid(ID, 0)), Some(std::sync::Arc::new(tribute_filter)), None);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(s.atkval)
    effect::set_value_fn(d, e1, effect_card_value(atkval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EFFECT_CLEAR_WORLD_IMMUNE)
    effect::set_code(d, e2, EFFECT_CLEAR_WORLD_IMMUNE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
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
    // e3:SetCondition(function(e) return e:GetHandler():IsTributeSummoned() end)
    effect::set_condition(d, e3, Some(condition(tribute_summoned_cond)));
    // e3:SetValue(s.actlimval)
    effect::set_value_fn(d, e3, lua_fn(actlimval));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(c) return c:ListsCode(CARD_CLEAR_WORLD) end` (the summon procedure's filter)
fn tribute_filter(d: &mut Duel, c: CardId, _tp: u8) -> bool {
    card::lists_code(d, c, &[CARD_CLEAR_WORLD])
}
/// `function(e) return e:GetHandler():IsTributeSummoned() end`
fn tribute_summoned_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_tribute_summoned(d, h)
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    // local _,val=Duel.GetMatchingGroup(Card.IsFaceup,e:GetHandlerPlayer(),0,LOCATION_MZONE,nil):GetMaxGroup(Card.GetBaseAttack)
    let p = effect::get_handler_player(d, e);
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), p, 0, LOCATION_MZONE, NO_EX);
    // return val or 0
    group::get_max_group(d, &g, &|d, c| card::get_base_attack(d, c) as i64).map_or(0, |(_, v)| v)
}
/// `s.actlimval(e,re,rp)`
fn actlimval(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let re = a.e(1);
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, re);
    // return re:IsMonsterEffect() and rc:IsSpecialSummoned() and rc:GetAttack()<e:GetHandler():GetAttack() and rc:IsFaceup()
    //   and rc:IsLocation(LOCATION_MZONE)
    effect::is_monster_effect(d, re)
        && card::is_special_summoned(d, rc.nil())
        && card::get_attack(d, rc.nil()) < card::get_attack(d, effect::get_handler(d, e).nil())
        && card::is_faceup(d, rc.nil())
        && card::is_location(d, rc.nil(), LOCATION_MZONE)
}
