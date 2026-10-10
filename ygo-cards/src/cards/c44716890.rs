//! Toll Hike
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 44716890;

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
    // Attack cost
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_ATTACK_COST)
    effect::set_code(d, e2, EFFECT_ATTACK_COST);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetTargetRange(1,1)
    effect::set_target_range(d, e2, 1, 1);
    // e2:SetCost(s.atcost)
    effect::set_cost(d, e2, Some(lua_fn(atcost)));
    // e2:SetOperation(s.atop)
    effect::set_operation(d, e2, Some(operation(atop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // accumulate
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(id)
    effect::set_code(d, e3, ID);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetTargetRange(1,1)
    effect::set_target_range(d, e3, 1, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.atcost(e,c,tp)`
fn atcost(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(2);
    // local ct=#{Duel.GetPlayerEffect(tp,id)}
    let ct = duel::get_player_effect(d, tp, ID).len();
    // return Duel.IsExistingMatchingCard(Card.IsAbleToGraveAsCost,tp,LOCATION_HAND,0,ct,nil)
    duel::is_existing_matching_card(d, Some(&card::is_able_to_grave_as_cost), tp, LOCATION_HAND, 0, ct as u32, NO_EX)
}
/// `s.atop(e,tp,eg,ep,ev,re,r,rp)`
fn atop(d: &mut Duel, a: &EffArgs) {
    // if Duel.IsAttackCostPaid()~=2 and e:GetHandler():IsLocation(LOCATION_SZONE) then
    if duel::is_attack_cost_paid(d) != 2 {
        let h = effect::get_handler(d, a.e).nil();
        if card::is_location(d, h, LOCATION_SZONE) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
            // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToGraveAsCost,tp,LOCATION_HAND,0,1,1,nil)
            let g = duel::select_matching_card(d, a.tp, Some(&card::is_able_to_grave_as_cost), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
            // Duel.SendtoGrave(g,REASON_COST)
            duel::send_to_grave(d, g, REASON_COST, None, None);
            // Duel.AttackCostPaid()
            duel::attack_cost_paid(d, None);
        }
    }
}
