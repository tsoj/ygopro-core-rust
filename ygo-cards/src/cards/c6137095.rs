//! Malfunction
use super::prelude::*;

pub const ID: u32 = 6137095;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_SET);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(500))));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return re:IsTrapEffect() and re:IsHasType(EFFECT_TYPE_ACTIVATE) and Duel.IsChainNegatable(ev)
    let re = a.re.nil();
    effect::is_trap_effect(d, re) && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && duel::is_chain_negatable(d, a.ev as u8)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    // if chk==0 then
    if a.chk == Some(0) {
        // if not re:GetHandler():IsStatus(STATUS_ACT_FROM_HAND) then
        let rc = effect::get_handler(d, re).nil();
        if !card::is_status(d, rc, STATUS_ACT_FROM_HAND) {
            // return re:GetHandler():IsCanTurnSet()
            return card::is_can_turn_set(d, rc);
        } else {
            return true;
        }
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // if re:GetHandler():IsRelateToEffect(re) and re:GetHandler():IsStatus(STATUS_ACT_FROM_HAND) then
    let rc = effect::get_handler(d, re).nil();
    if card::is_relate_to_effect(d, rc, re) && card::is_status(d, rc, STATUS_ACT_FROM_HAND) {
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, a.eg.clone(), 1, 0, 0);
    }
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // if Duel.NegateActivation(ev) and rc:IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) && card::is_relate_to_effect(d, rc, re) {
        // if rc:IsStatus(STATUS_ACT_FROM_HAND) then
        if card::is_status(d, rc, STATUS_ACT_FROM_HAND) {
            // rc:CancelToGrave()
            card::cancel_to_grave(d, rc, true);
            // Duel.SendtoHand(rc,nil,REASON_EFFECT)
            duel::send_to_hand(d, rc, None, REASON_EFFECT, None);
        } else {
            // if rc:IsCanTurnSet() then
            if card::is_can_turn_set(d, rc) {
                // rc:CancelToGrave()
                card::cancel_to_grave(d, rc, true);
                // Duel.ChangePosition(rc,POS_FACEDOWN)
                duel::change_position(d, rc, POS_FACEDOWN as u8, None, None, None, false);
                // rc:SetStatus(STATUS_ACTIVATE_DISABLED,false)
                card::set_status(d, rc, STATUS_ACTIVATE_DISABLED, false);
                // rc:SetStatus(STATUS_SET_TURN,false)
                card::set_status(d, rc, STATUS_SET_TURN, false);
                // Duel.RaiseEvent(rc,EVENT_SSET,e,REASON_EFFECT,tp,tp,0)
                duel::raise_event(d, rc, EVENT_SSET, Some(a.e), REASON_EFFECT, a.tp, a.tp, 0);
            }
        }
    }
}
