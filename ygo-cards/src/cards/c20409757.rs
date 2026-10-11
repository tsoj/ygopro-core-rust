//! Timegazer Magician
use super::prelude::*;

pub const ID: u32 = 20409757;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MAGICIAN, SET_ODD_EYES], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // pendulum summon
    // Pendulum.AddProcedure(c,false)
    pendulum::add_procedure(d, c, Some(false), None);
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(1160)
    effect::set_description(d, e1, 1160);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // actlimit
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetRange(LOCATION_PZONE)
    effect::set_range(d, e2, LOCATION_PZONE);
    // e2:SetCondition(s.actcon)
    effect::set_condition(d, e2, Some(condition(actcon)));
    // e2:SetOperation(s.actop)
    effect::set_operation(d, e2, Some(operation(actop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e3, EVENT_BE_BATTLE_TARGET);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // scale
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetCode(EFFECT_CHANGE_LSCALE)
    effect::set_code(d, e4, EFFECT_CHANGE_LSCALE);
    // e4:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e4:SetRange(LOCATION_PZONE)
    effect::set_range(d, e4, LOCATION_PZONE);
    // e4:SetCondition(s.slcon)
    effect::set_condition(d, e4, Some(condition(slcon)));
    // e4:SetValue(4)
    effect::set_value(d, e4, 4);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=e4:Clone()
    let e5 = effect::clone(d, e4);
    // e5:SetCode(EFFECT_CHANGE_RSCALE)
    effect::set_code(d, e5, EFFECT_CHANGE_RSCALE);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // indes
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e6, EFFECT_TYPE_FIELD);
    // e6:SetCode(EFFECT_INDESTRUCTABLE_COUNT)
    effect::set_code(d, e6, EFFECT_INDESTRUCTABLE_COUNT);
    // e6:SetRange(LOCATION_MZONE)
    effect::set_range(d, e6, LOCATION_MZONE);
    // e6:SetTargetRange(LOCATION_PZONE,0)
    effect::set_target_range(d, e6, LOCATION_PZONE, 0);
    // e6:SetTarget(aux.TRUE)
    effect::set_target(d, e6, Some(effect_card_fn(|_d, _e, _c| true)));
    // e6:SetValue(s.indval)
    effect::set_value_fn(d, e6, lua_fn(indval));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)==0
    duel::get_field_group_count(d, a.tp, LOCATION_MZONE, 0) == 0
}
/// `s.actcon(e,tp,eg,ep,ev,re,r,rp)`
fn actcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=Duel.GetAttacker()
    let mut tc = duel::get_attacker(d);
    // if tc:IsControler(1-tp) then tc=Duel.GetAttackTarget() end
    if card::is_controler(d, tc.nil(), 1 - a.tp) {
        tc = duel::get_attack_target(d);
    }
    // return tc and tc:IsControler(tp) and tc:IsType(TYPE_PENDULUM)
    match tc {
        Some(tc) => card::is_controler(d, tc, a.tp) && card::is_type(d, tc, TYPE_PENDULUM),
        None => false,
    }
}
/// `s.actop(e,tp,eg,ep,ev,re,r,rp)`
fn actop(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    // e1:SetTargetRange(0,1)
    effect::set_target_range(d, e1, 0, 1);
    // e1:SetValue(s.aclimit)
    effect::set_value_fn(d, e1, lua_fn(aclimit));
    // e1:SetReset(RESET_PHASE|PHASE_DAMAGE)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `s.aclimit(e,re,tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    // return re:IsTrapEffect() and re:IsHasType(EFFECT_TYPE_ACTIVATE)
    let re = a.e(1);
    effect::is_trap_effect(d, re) && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
}
/// `s.slfilter(c)`
fn slfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard({SET_MAGICIAN,SET_ODD_EYES})
    card::is_set_card(d, c, &[SET_MAGICIAN as u16, SET_ODD_EYES as u16])
}
/// `s.slcon(e)`
fn slcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsExistingMatchingCard(s.slfilter,e:GetHandlerPlayer(),LOCATION_PZONE,0,1,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let p = effect::get_handler_player(d, a.e);
    !duel::is_existing_matching_card(d, Some(&slfilter), p, LOCATION_PZONE, 0, 1, h.into())
}
/// `s.indval(e,re,r,rp)`
fn indval(d: &mut Duel, a: Args) -> i64 {
    let e = a.e(0);
    let r = a.int(2);
    let rp = a.int(3);
    // if (r&REASON_EFFECT)~=0 and rp==1-e:GetHandlerPlayer() then
    if (r & REASON_EFFECT as i64) != 0 && rp == 1 - effect::get_handler_player(d, e) as i64 {
        // return 1
        1
    } else {
        // else return 0 end
        0
    }
}
