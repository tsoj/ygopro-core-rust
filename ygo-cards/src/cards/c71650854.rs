//! 半魔導帯域 (Magical Mid-Breaker Field)
use super::prelude::*;

pub const ID: u32 = 71650854;

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
    // Prevent effect target
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e2:SetProperty(EFFECT_FLAG_SET_AVAILABLE+EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_SET_AVAILABLE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetCondition(s.indcon)
    effect::set_condition(d, e2, Some(condition(indcon)));
    // e2:SetValue(aux.tgoval)
    effect::set_value_fn(d, e2, aux::tgoval());
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Prevent destruction by opponent's effect
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_EFFECT);
    // e3:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e3, EFFECT_FLAG_SET_AVAILABLE, 0);
    // e3:SetValue(aux.indoval)
    effect::set_value_fn(d, e3, aux::indoval());
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e4, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e4:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e4, 0, LOCATION_MZONE);
    // e4:SetValue(s.tgoval)
    effect::set_value_fn(d, e4, lua_fn(tgoval));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=e3:Clone()
    let e5 = effect::clone(d, e3);
    // e5:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e5, EFFECT_INDESTRUCTABLE_EFFECT);
    // e5:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e5, 0, LOCATION_MZONE);
    // e5:SetValue(s.tgovalue)
    effect::set_value_fn(d, e5, lua_fn(tgovalue));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // cannot set/activate
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e6, EFFECT_TYPE_FIELD);
    // e6:SetCode(EFFECT_CANNOT_SSET)
    effect::set_code(d, e6, EFFECT_CANNOT_SSET);
    // e6:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e6, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e6:SetRange(LOCATION_FZONE)
    effect::set_range(d, e6, LOCATION_FZONE);
    // e6:SetTargetRange(1,0)
    effect::set_target_range(d, e6, 1, 0);
    // e6:SetTarget(s.setlimit)
    effect::set_target(d, e6, Some(lua_fn(setlimit)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
    // local e7=Effect.CreateEffect(c)
    let e7 = effect::create_effect(d, c);
    // e7:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e7, EFFECT_TYPE_FIELD);
    // e7:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e7, EFFECT_CANNOT_ACTIVATE);
    // e7:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e7, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e7:SetRange(LOCATION_FZONE)
    effect::set_range(d, e7, LOCATION_FZONE);
    // e7:SetTargetRange(1,0)
    effect::set_target_range(d, e7, 1, 0);
    // e7:SetValue(s.actlimit)
    effect::set_value_fn(d, e7, lua_fn(actlimit));
    // c:RegisterEffect(e7)
    card::register_effect(d, c, e7, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return Duel.IsTurnPlayer(tp) and (Duel.IsMainPhase())
    //     and (not Duel.CheckPhaseActivity() or (Duel.GetFlagEffect(tp,CARD_MAGICAL_MIDBREAKER)>0 and Duel.GetCurrentChain()==1))
    duel::is_turn_player(d, tp)
        && duel::is_main_phase(d, None)
        && (!duel::check_phase_activity(d) || (duel::get_flag_effect(d, tp, CARD_MAGICAL_MIDBREAKER) > 0 && duel::get_current_chain(d, false) == 1))
}
// function s.indcon(e,tp,eg,ep,ev,re,r,rp)
fn indcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsPhase(PHASE_MAIN1)
    duel::is_phase(d, PHASE_MAIN1)
}
// function s.tgoval(e,re,rp)
fn tgoval(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let rp = a.int(2);
    // return rp~=1-e:GetHandlerPlayer() and not re:GetHandler():IsImmuneToEffect(e)
    let re = a.e(1);
    rp != 1 - effect::get_handler_player(d, e) as i64 && {
        let rc = effect::get_handler(d, re).nil();
        !card::is_immune_to_effect(d, rc, e)
    }
}
// function s.tgovalue(e,re,rp)
fn tgovalue(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let rp = a.int(2);
    // return rp~=1-e:GetHandlerPlayer()
    rp != 1 - effect::get_handler_player(d, e) as i64
}
// function s.setlimit(e,c,tp)
fn setlimit(d: &mut Duel, a: Args) -> bool {
    // return c:IsType(TYPE_FIELD)
    let c = a.c(1);
    card::is_type(d, c, TYPE_FIELD)
}
// function s.actlimit(e,re,tp)
fn actlimit(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    // return re:IsActiveType(TYPE_FIELD) and re:IsHasType(EFFECT_TYPE_ACTIVATE)
    effect::is_active_type(d, re, TYPE_FIELD) && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
}
