//! Coach King Giantrainer
use super::prelude::*;

pub const ID: u32 = 30741334;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz.AddProcedure(c,nil,8,3)
    xyz::add_procedure(d, c, None, Some(8), 3, None, None, None, None, false, None);
    // Draw 1 card and reveal it, then if it was a monster, inflict 800 damage to your opponent
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DRAW+CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DRAW + CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(3,id)
    effect::set_count_limit(d, e1, 3, ID, 0, 0);
    // e1:SetCost(Cost.AND(Cost.DetachFromSelf(1),s.drcost))
    effect::set_cost(d, e1, Some(cost::and(vec![cost::detach_from_self(cost::Count::N(1), None, None), cost_target(drcost)])));
    // e1:SetTarget(s.drtg)
    effect::set_target(d, e1, Some(cost_target(drtg)));
    // e1:SetOperation(s.drop)
    effect::set_operation(d, e1, Some(operation(drop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.drcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPhase(PHASE_MAIN1) end
    if a.chk == Some(0) {
        return duel::is_phase(d, PHASE_MAIN1);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // aux.RegisterClientHint(c,nil,tp,1,0,aux.Stringid(id,1))
    aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
    // You cannot conduct your Battle Phase the turn you activate this effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH, 0);
    // e1:SetCode(EFFECT_CANNOT_BP)
    effect::set_code(d, e1, EFFECT_CANNOT_BP);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    true
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DAMAGE,nil,1,1-tp,800)
    duel::set_possible_operation_info(d, 0, CATEGORY_DAMAGE, None, 1, 1 - a.tp, 800);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drop(d: &mut Duel, a: &EffArgs) {
    // local ct=Duel.Draw(tp,1,REASON_EFFECT)
    let ct = duel::draw(d, a.tp, 1, REASON_EFFECT);
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // local dc=Duel.GetOperatedGroup():GetFirst()
    let operated = duel::get_operated_group(d);
    let dc = group::get_first(d, &operated).nil();
    // Duel.ConfirmCards(1-tp,dc)
    duel::confirm_cards(d, 1 - a.tp, dc);
    // if dc:IsMonster() then
    if card::is_monster(d, dc) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Damage(1-tp,800,REASON_EFFECT)
        duel::damage(d, 1 - a.tp, 800, REASON_EFFECT, false, None);
    }
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, a.tp);
}
