//! Summon Limit
use super::prelude::*;

pub const ID: u32 = 23516703;

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
    // summon count limit
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EFFECT_CANNOT_SUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_SUMMON);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetTargetRange(1,1)
    effect::set_target_range(d, e2, 1, 1);
    // e2:SetTarget(s.limittg)
    effect::set_target(d, e2, Some(lua_fn(limittg)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_CANNOT_FLIP_SUMMON)
    effect::set_code(d, e3, EFFECT_CANNOT_FLIP_SUMMON);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e2:Clone()
    let e4 = effect::clone(d, e2);
    // e4:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e4, EFFECT_CANNOT_SPECIAL_SUMMON);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // counter
    // local et=Effect.CreateEffect(c)
    let et = effect::create_effect(d, c);
    // et:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, et, EFFECT_TYPE_FIELD);
    // et:SetCode(EFFECT_LEFT_SPSUMMON_COUNT)
    effect::set_code(d, et, EFFECT_LEFT_SPSUMMON_COUNT);
    // et:SetRange(LOCATION_SZONE)
    effect::set_range(d, et, LOCATION_SZONE);
    // et:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, et, EFFECT_FLAG_PLAYER_TARGET, 0);
    // et:SetTargetRange(1,1)
    effect::set_target_range(d, et, 1, 1);
    // et:SetValue(s.countval)
    effect::set_value_fn(d, et, lua_fn(countval));
    // c:RegisterEffect(et)
    card::register_effect(d, c, et, false);
}
/// `s.limittg(e,c,tp)`
fn limittg(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(2);
    // local t1,t2,t3=Duel.GetActivityCount(tp,ACTIVITY_SUMMON,ACTIVITY_FLIPSUMMON,ACTIVITY_SPSUMMON)
    let t = duel::get_activity_count(d, tp, &[ACTIVITY_SUMMON, ACTIVITY_FLIPSUMMON, ACTIVITY_SPSUMMON]);
    // return t1+t2+t3>=2
    t[0] + t[1] + t[2] >= 2
}
/// `s.countval(e,re,tp)`
fn countval(d: &mut Duel, a: Args) -> i64 {
    let tp = a.p(2);
    // local t1,t2,t3=Duel.GetActivityCount(tp,ACTIVITY_SUMMON,ACTIVITY_FLIPSUMMON,ACTIVITY_SPSUMMON)
    let t = duel::get_activity_count(d, tp, &[ACTIVITY_SUMMON, ACTIVITY_FLIPSUMMON, ACTIVITY_SPSUMMON]);
    // if t1+t2+t3>=2 then return 0 else return 2-t1-t2-t3 end
    if t[0] + t[1] + t[2] >= 2 {
        0
    } else {
        2 - t[0] - t[1] - t[2]
    }
}
