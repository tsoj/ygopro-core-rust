//! Quiet Life
//! スローライフ
use super::prelude::*;

pub const ID: u32 = 31980955;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // summon limit
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EFFECT_CANNOT_SUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_SUMMON);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(1,1)
    effect::set_target_range(d, e2, 1, 1);
    // e2:SetTarget(s.sumlimit1)
    effect::set_target(d, e2, Some(lua_fn(sumlimit1)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_CANNOT_MSET)
    effect::set_code(d, e3, EFFECT_CANNOT_MSET);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e2:Clone()
    let e4 = effect::clone(d, e2);
    // e4:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e4, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e4:SetTarget(s.sumlimit2)
    effect::set_target(d, e4, Some(lua_fn(sumlimit2)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return Duel.IsPhase(PHASE_MAIN1) and not Duel.CheckPhaseActivity()
    //     and Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)==0
    duel::is_phase(d, PHASE_MAIN1)
        && !duel::check_phase_activity(d)
        && duel::get_field_group_count(d, tp, LOCATION_MZONE, 0) == 0
}
/// `s.sumlimit1(e,c,sump,sumtype,sumpos,targetp,se)`
fn sumlimit1(d: &mut Duel, a: Args) -> bool {
    // return Duel.GetActivityCount(sump,ACTIVITY_SPSUMMON)>0
    duel::get_activity_count(d, a.p(2), &[ACTIVITY_SPSUMMON])[0] > 0
}
/// `s.sumlimit2(e,c,sump,sumtype,sumpos,targetp,se)`
fn sumlimit2(d: &mut Duel, a: Args) -> bool {
    // return Duel.GetActivityCount(sump,ACTIVITY_NORMALSUMMON)>0
    duel::get_activity_count(d, a.p(2), &[ACTIVITY_NORMALSUMMON])[0] > 0
}
