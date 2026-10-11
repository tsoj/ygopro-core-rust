//! EMホタルクス
//! Performapal Fireflux
use super::prelude::*;

pub const ID: u32 = 12255007;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PERFORMAPAL, SET_ODD_EYES], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // pendulum summon
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // actlimit
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(0,1)
    effect::set_target_range(d, e1, 0, 1);
    // e1:SetValue(s.aclimit)
    effect::set_value_fn(d, e1, lua_fn(aclimit));
    // e1:SetCondition(s.actcon)
    effect::set_condition(d, e1, Some(lua_fn(actcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // negate attack
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetRange(LOCATION_PZONE)
    effect::set_range(d, e2, LOCATION_PZONE);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetCost(s.cost)
    effect::set_cost(d, e2, Some(cost_target(cost_fn)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn aclimit(d: &mut Duel, a: Args) -> bool {
    // return not re:GetHandler():IsImmuneToEffect(e) and re:IsHasType(EFFECT_TYPE_ACTIVATE)
    let e = a.e(0);
    let re = a.e(1);
    let h = effect::get_handler(d, re).nil();
    !card::is_immune_to_effect(d, h, e) && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
}
fn actcon(d: &mut Duel, a: Args) -> bool {
    // local tc=Duel.GetAttacker()
    let tc = duel::get_attacker(d);
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e(0));
    // return tc and tc:IsControler(tp) and (tc:IsSetCard(SET_PERFORMAPAL) or tc:IsSetCard(SET_ODD_EYES))
    match tc {
        Some(tc) => card::is_controler(d, tc, tp) && (card::is_set_card(d, tc, &[SET_PERFORMAPAL as u16]) || card::is_set_card(d, tc, &[SET_ODD_EYES as u16])),
        None => false,
    }
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp)
    duel::is_turn_player(d, 1 - a.tp)
}
fn is_perf(d: &mut Duel, c: CardId) -> bool {
    // Card.IsSetCard with the extra argument SET_PERFORMAPAL
    card::is_set_card(d, c, &[SET_PERFORMAPAL as u16])
}
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,Card.IsSetCard,1,false,nil,nil,SET_PERFORMAPAL) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&is_perf), 1, 1, false, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,Card.IsSetCard,1,1,false,nil,nil,SET_PERFORMAPAL)
    let g = duel::select_release_group_cost(d, a.tp, Some(&is_perf), 1, 1, false, None, NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():IsRelateToEffect(e) and Duel.NegateAttack() then
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) && duel::negate_attack(d) {
        // Duel.SkipPhase(1-tp,PHASE_BATTLE,RESET_PHASE|PHASE_BATTLE_STEP,1)
        duel::skip_phase(d, 1 - a.tp, PHASE_BATTLE, RESET_PHASE | PHASE_BATTLE_STEP, 1, 0);
    }
}
