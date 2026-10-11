//! Narrow Pass
//! 狭き通路
use super::prelude::*;

pub const ID: u32 = 40172183;

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
    // e2:SetTarget(s.sumlimit)
    effect::set_target(d, e2, Some(lua_fn(|d, a| sumlimit(d, a))));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)<=2
    //     and Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)<=2
    duel::get_field_group_count(d, a.tp, LOCATION_MZONE, 0) <= 2 && duel::get_field_group_count(d, a.tp, 0, LOCATION_MZONE) <= 2
}
/// `s.sumlimit(e,c,sp,st)`
fn sumlimit(d: &mut Duel, a: Args<'_>) -> bool {
    // return Duel.GetFieldGroupCount(sp,LOCATION_MZONE,0)>=2
    duel::get_field_group_count(d, a.p(2), LOCATION_MZONE, 0) >= 2
}
