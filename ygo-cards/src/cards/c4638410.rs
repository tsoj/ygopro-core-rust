//! Tyrant's Temper
use super::prelude::*;

pub const ID: u32 = 4638410;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // immune
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e2, EFFECT_IMMUNE_EFFECT);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    // e2:SetTarget(s.etarget)
    effect::set_target(d, e2, Some(effect_card_fn(etarget)));
    // e2:SetValue(s.efilter)
    effect::set_value_fn(d, e2, lua_fn(|d, a: Args| efilter(d, a.e(0), a.e(1))));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,aux.TRUE,1,false,nil,nil,tp) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, Some(&|_, _| true), 1, 1, false, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,aux.TRUE,1,1,false,nil,nil,tp)
    let g = duel::select_release_group_cost(d, tp, Some(&|_, _| true), 1, 1, false, None, NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.etarget(e,c)`
fn etarget(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:GetOwner()==e:GetHandlerPlayer()
    card::get_owner(d, c) == effect::get_handler_player(d, e)
}
/// `s.efilter(e,te)`
fn efilter(d: &mut Duel, e: EffectId, te: EffectId) -> bool {
    // return te:IsTrapEffect() and te:GetOwner()~=e:GetOwner()
    effect::is_trap_effect(d, te) && effect::get_owner(d, te) != effect::get_owner(d, e)
}
