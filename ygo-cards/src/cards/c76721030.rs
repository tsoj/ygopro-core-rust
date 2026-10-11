//! Tyrant's Tirade
use super::prelude::*;

pub const ID: u32 = 76721030;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // e0:SetCost(s.cost)
    effect::set_cost(d, e0, Some(cost_target(cost_fn)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Effects of Effect Monsters that activate in the hand or on the field cannot be activated
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetTargetRange(1,1)
    effect::set_target_range(d, e1, 1, 1);
    // e1:SetValue(function(e,re,tp) return re:IsMonsterEffect() and re:GetHandler():IsLocation(LOCATION_HAND|LOCATION_MZONE) end)
    effect::set_value_fn(d, e1, lua_fn(act_limit));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,nil,2,false,nil,nil) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, None, 2, 2, false, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,nil,2,2,false,nil,nil)
    let g = duel::select_release_group_cost(d, a.tp, None, 2, 2, false, None, NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// The value function `function(e,re,tp)`
fn act_limit(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    // return re:IsMonsterEffect() and re:GetHandler():IsLocation(LOCATION_HAND|LOCATION_MZONE)
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    let rh = effect::get_handler(d, re).nil();
    card::is_location(d, rh, LOCATION_HAND | LOCATION_MZONE)
}
