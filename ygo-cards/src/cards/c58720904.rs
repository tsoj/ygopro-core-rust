//! Pendransaction
use super::prelude::*;

pub const ID: u32 = 58720904;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Xyz Summon
    // Xyz.AddProcedure(c,nil,4,2)
    xyz::add_procedure(d, c, None, Some(4), 2, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Apply effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,LOCATION_EXTRA,0)-Duel.GetFieldGroupCount(tp,0,LOCATION_EXTRA)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, LOCATION_EXTRA, 0) - duel::get_field_group_count(d, a.tp, 0, LOCATION_EXTRA) > 0;
    }
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local val=Duel.GetFieldGroupCount(tp,LOCATION_EXTRA,0)-Duel.GetFieldGroupCount(tp,0,LOCATION_EXTRA)
    let val = duel::get_field_group_count(d, a.tp, LOCATION_EXTRA, 0) - duel::get_field_group_count(d, a.tp, 0, LOCATION_EXTRA);
    // Increase ATK
    // if val>=1 and c:IsRelateToEffect(e) and c:IsFaceup() then
    if val >= 1 && card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END,2)
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 2);
        // e1:SetValue(1000)
        effect::set_value(d, e1, 1000);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
    // Cannot be targeted by effects
    // if val>=5 and c:IsRelateToEffect(e) and c:IsFaceup() then
    if val >= 5 && card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
        effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
        // e2:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
        effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
        // e2:SetRange(LOCATION_MZONE)
        effect::set_range(d, e2, LOCATION_MZONE);
        // e2:SetReset(RESETS_STANDARD_DISABLE_PHASE_END,2)
        effect::set_reset(d, e2, RESETS_STANDARD_DISABLE_PHASE_END, 2);
        // e2:SetValue(1)
        effect::set_value(d, e2, 1);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
    }
    // Banish 1 card the opponent controls
    // if val>=10 then
    if val >= 10 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
        // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&|d, x| card::is_able_to_remove(d, x, None, None, None)), a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
        // if #g>0 then Duel.Remove(g,POS_FACEUP,REASON_EFFECT) end
        if !g.is_empty() {
            duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
    // Change opponent's LP to 3000
    // if val>=15 then
    if val >= 15 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SetLP(1-tp,3000)
        duel::set_lp(d, 1 - a.tp, 3000);
    }
}
