//! Terminal World NEXT
//! 端末世界NEXT
use super::prelude::*;

pub const ID: u32 = 48605591;

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
    // Cards cannot be placed on the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EFFECT_MAX_MZONE)
    effect::set_code(d, e2, EFFECT_MAX_MZONE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(1,1)
    effect::set_target_range(d, e2, 1, 1);
    // e2:SetValue(s.mvalue)
    effect::set_value_fn(d, e2, lua_fn(mvalue));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetCode(EFFECT_MAX_SZONE)
    effect::set_code(d, e3, EFFECT_MAX_SZONE);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetTargetRange(1,1)
    effect::set_target_range(d, e3, 1, 1);
    // e3:SetValue(s.svalue)
    effect::set_value_fn(d, e3, lua_fn(svalue));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e4:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e4, EFFECT_CANNOT_ACTIVATE);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetTargetRange(1,1)
    effect::set_target_range(d, e4, 1, 1);
    // e4:SetValue(s.aclimit)
    effect::set_value_fn(d, e4, lua_fn(aclimit));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD);
    // e5:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e5, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e5:SetCode(EFFECT_CANNOT_SSET)
    effect::set_code(d, e5, EFFECT_CANNOT_SSET);
    // e5:SetRange(LOCATION_SZONE)
    effect::set_range(d, e5, LOCATION_SZONE);
    // e5:SetTargetRange(1,1)
    effect::set_target_range(d, e5, 1, 1);
    // e5:SetTarget(s.setlimit)
    effect::set_target(d, e5, Some(lua_fn(setlimit)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsExistingMatchingCard(nil,tp,LOCATION_ONFIELD,0,1,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    !duel::is_existing_matching_card(d, None, a.tp, LOCATION_ONFIELD, 0, 1, h.into())
        // and Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)<=3
        && duel::get_field_group_count(d, a.tp, 0, LOCATION_MZONE) <= 3
        // and Duel.GetMatchingGroupCount(Card.IsSpellTrap,tp,0,LOCATION_ONFIELD,nil)<=3
        && duel::get_matching_group_count(d, Some(&card::is_spell_trap), a.tp, 0, LOCATION_ONFIELD, NO_EX) <= 3
}
// function s.mvalue(e,fp,rp,r)
fn mvalue(_d: &mut Duel, a: Args) -> i64 {
    // if r~=LOCATION_REASON_TOFIELD then return 99 end
    if a.int(3) as u32 != LOCATION_REASON_TOFIELD {
        return 99;
    }
    // return 3
    3
}
// function s.svalue(e,fp,rp,r)
fn svalue(d: &mut Duel, a: Args) -> i64 {
    // if r~=LOCATION_REASON_TOFIELD then return 99 end
    if a.int(3) as u32 != LOCATION_REASON_TOFIELD {
        return 99;
    }
    // return 3-Duel.GetFieldGroupCount(fp,LOCATION_FZONE,0)
    3 - duel::get_field_group_count(d, a.p(1), LOCATION_FZONE, 0) as i64
}
// function s.aclimit(e,re,tp)
fn aclimit(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    let tp = a.p(2);
    // if not re:IsHasType(EFFECT_TYPE_ACTIVATE) then return false end
    if !effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) {
        return false;
    }
    // if re:IsActiveType(TYPE_FIELD) then
    if effect::is_active_type(d, re, TYPE_FIELD) {
        // return not Duel.GetFieldCard(tp,LOCATION_FZONE,0) and Duel.GetFieldGroupCount(tp,LOCATION_SZONE,0)>2
        return duel::get_field_card(d, tp, LOCATION_FZONE, 0).is_none() && duel::get_field_group_count(d, tp, LOCATION_SZONE, 0) > 2;
    // elseif re:IsActiveType(TYPE_PENDULUM) then
    } else if effect::is_active_type(d, re, TYPE_PENDULUM) {
        // return Duel.GetFieldGroupCount(tp,LOCATION_SZONE,0)>2
        return duel::get_field_group_count(d, tp, LOCATION_SZONE, 0) > 2;
    }
    // return false
    false
}
// function s.setlimit(e,c,tp)
fn setlimit(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    let tp = a.p(2);
    // return c:IsFieldSpell() and not Duel.GetFieldCard(tp,LOCATION_FZONE,0) and Duel.GetFieldGroupCount(tp,LOCATION_SZONE,0)>2
    card::is_field_spell(d, c) && duel::get_field_card(d, tp, LOCATION_FZONE, 0).is_none() && duel::get_field_group_count(d, tp, LOCATION_SZONE, 0) > 2
}
