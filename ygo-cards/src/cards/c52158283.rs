//! Chronomaly Colossal Head
use super::prelude::*;

pub const ID: u32 = 52158283;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // adchange
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_POSITION+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_POSITION + CATEGORY_SET);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e1, Some(cost::self_banish()));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttackPos() and c:IsLevelAbove(3)
    card::is_attack_pos(d, c) && card::is_level_above(d, c, 3)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_POSCHANGE as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,g,1,tp,POS_FACEUP_DEFENSE|POS_FACEDOWN_DEFENSE)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, g, 1, a.tp, (POS_FACEUP_DEFENSE | POS_FACEDOWN_DEFENSE) as i32);
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // if tc:IsAttackPos() then
        if card::is_attack_pos(d, tc) {
            // local pos=0
            // if tc:IsCanTurnSet() then
            let pos = if card::is_can_turn_set(d, tc) {
                // pos=Duel.SelectPosition(tp,tc,POS_DEFENSE)
                duel::select_position(d, a.tp, tc, POS_DEFENSE as u8)
            } else {
                // pos=Duel.SelectPosition(tp,tc,POS_FACEUP_DEFENSE)
                duel::select_position(d, a.tp, tc, POS_FACEUP_DEFENSE as u8)
            };
            // Duel.ChangePosition(tc,pos)
            duel::change_position(d, tc, pos as u8, None, None, None, false);
        } else {
            // Duel.ChangePosition(tc,0,0,POS_FACEDOWN_DEFENSE,POS_FACEUP_DEFENSE)
            duel::change_position(d, tc, 0, Some(0), Some(POS_FACEDOWN_DEFENSE as u8), Some(POS_FACEUP_DEFENSE as u8), false);
        }
    }
}
