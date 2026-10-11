//! Zap Mustung
//! 放電ムスタンガン
use super::prelude::*;

pub const ID: u32 = 29951323;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableUnsummonable()
    card::enable_unsummonable(d, c);
    // splimit
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(s.splimit)
    effect::set_value_fn(d, e1, lua_fn(splimit));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // indes
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_COUNT)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_COUNT);
    // e2:SetCountLimit(2)
    effect::set_count_limit(d, e2, 2, 0, 0, 0);
    // e2:SetValue(s.valcon)
    effect::set_value_fn(d, e2, lua_fn(valcon));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // spsummon limit
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e3, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetTargetRange(1,1)
    effect::set_target_range(d, e3, 1, 1);
    // e3:SetTarget(s.limittg)
    effect::set_target(d, e3, Some(lua_fn(limittg)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // counter
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetCode(EFFECT_LEFT_SPSUMMON_COUNT)
    effect::set_code(d, e4, EFFECT_LEFT_SPSUMMON_COUNT);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e4:SetTargetRange(1,0)
    effect::set_target_range(d, e4, 1, 0);
    // e4:SetCondition(s.countcon1)
    effect::set_condition(d, e4, Some(lua_fn(countcon1)));
    // e4:SetValue(s.countval)
    effect::set_value_fn(d, e4, lua_fn(countval));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=e4:Clone()
    let e5 = effect::clone(d, e4);
    // e5:SetCondition(s.countcon2)
    effect::set_condition(d, e5, Some(lua_fn(countcon2)));
    // e5:SetTargetRange(0,1)
    effect::set_target_range(d, e5, 0, 1);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
// function s.splimit(e,se,sp,st)
fn splimit(d: &mut Duel, a: Args) -> bool {
    // return se:IsHasType(EFFECT_TYPE_ACTIONS) and Duel.GetActivityCount(e:GetHandlerPlayer(),ACTIVITY_SPSUMMON)==0
    //     and Duel.IsPhase(PHASE_MAIN1) and Duel.GetTurnPlayer()==e:GetHandlerPlayer()
    let se = a.e(1);
    let hp = effect::get_handler_player(d, a.e(0));
    effect::is_has_type(d, se, EFFECT_TYPE_ACTIONS)
        && duel::get_activity_count(d, hp, &[ACTIVITY_SPSUMMON])[0] == 0
        && duel::is_phase(d, PHASE_MAIN1)
        && duel::get_turn_player(d) == effect::get_handler_player(d, a.e(0))
}
// function s.valcon(e,re,r,rp)
fn valcon(_d: &mut Duel, a: Args) -> bool {
    // return (r&REASON_BATTLE)~=0
    ((a.int(2) as u32) & REASON_BATTLE) != 0
}
// function s.limittg(e,c,tp)
fn limittg(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(2);
    // return Duel.IsTurnPlayer(tp) and Duel.GetActivityCount(tp,ACTIVITY_SPSUMMON)>=Duel.GetBattledCount(tp)
    duel::is_turn_player(d, tp) && duel::get_activity_count(d, tp, &[ACTIVITY_SPSUMMON])[0] >= duel::get_battled_count(d, tp) as i64
}
// function s.countcon1(e)
fn countcon1(d: &mut Duel, a: Args) -> bool {
    // return Duel.GetTurnPlayer()==e:GetHandlerPlayer()
    duel::get_turn_player(d) == effect::get_handler_player(d, a.e(0))
}
// function s.countcon2(e)
fn countcon2(d: &mut Duel, a: Args) -> bool {
    // return Duel.GetTurnPlayer()~=e:GetHandlerPlayer()
    duel::get_turn_player(d) != effect::get_handler_player(d, a.e(0))
}
// function s.countval(e,re,tp)
fn countval(d: &mut Duel, a: Args) -> i64 {
    let tp = a.p(2);
    // local t1=Duel.GetActivityCount(tp,ACTIVITY_SPSUMMON)
    let t1 = duel::get_activity_count(d, tp, &[ACTIVITY_SPSUMMON])[0];
    // local t2=Duel.GetBattledCount(tp)
    let t2 = duel::get_battled_count(d, tp) as i64;
    // if t1>=t2 then return 0 else return t2-t1 end
    if t1 >= t2 { 0 } else { t2 - t1 }
}
