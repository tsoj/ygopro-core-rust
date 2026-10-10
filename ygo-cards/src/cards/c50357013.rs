//! Tokusano Shinkyojin
use super::prelude::*;

pub const ID: u32 = 50357013;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:HasLevel() and c:IsAbleToGraveAsCost() and (c:IsFaceup() or c:IsLocation(LOCATION_HAND))
    card::has_level(d, c) && card::is_able_to_grave_as_cost(d, c) && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_HAND))
}
/// s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&cfilter), a.tp, LOCATION_HAND | LOCATION_MZONE, 0, NO_EX);
    // local ct=#g
    let ct = g.len() as i32;
    // if chk==0 then return ct>0 and g:CheckWithSumEqual(Card.GetLevel,10,1,ct) end
    if a.chk == Some(0) {
        return ct > 0 && group::check_with_sum_equal(d, &g, &|d, x| card::get_level(d, x) as i64, 10, 1, ct).0;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local cg=g:SelectWithSumEqual(tp,Card.GetLevel,10,1,ct)
    let cg = group::select_with_sum_equal(d, &g, a.tp, &|d, x| card::get_level(d, x) as i64, 10, 1, ct).nil();
    // Duel.SendtoGrave(cg,REASON_COST)
    duel::send_to_grave(d, cg, REASON_COST, None, None);
    true
}
/// s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,2) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 2);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(2)
    duel::set_target_param(d, 2);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,2)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 2);
    true
}
/// s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
