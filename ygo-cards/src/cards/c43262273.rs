//! Appointer of the Red Lotus
use super::prelude::*;

pub const ID: u32 = 43262273;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_TOHAND)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_TOHAND as u32));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckLPCost(tp,2000) and Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)~=0
    //   and not Duel.IsExistingMatchingCard(Card.IsPublic,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::check_lp_cost(d, a.tp, 2000)
            && duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0) != 0
            && !duel::is_existing_matching_card(d, Some(&|d, c| card::is_public(d, c)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.PayLPCost(tp,2000)
    duel::pay_lp_cost(d, a.tp, 2000);
    // local g=Duel.GetFieldGroup(tp,LOCATION_HAND,0)
    let g = duel::get_field_group(d, a.tp, LOCATION_HAND, 0);
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - a.tp, g);
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, a.tp);
    false
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)~=0
    //   and Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,0,LOCATION_HAND,1,nil) end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND) != 0
            && duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, 0, LOCATION_HAND, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,0,1-tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 0, 1 - a.tp, LOCATION_HAND as i32);
    false
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g0=Duel.GetFieldGroup(tp,0,LOCATION_HAND)
    let g0 = duel::get_field_group(d, a.tp, 0, LOCATION_HAND);
    // Duel.ConfirmCards(tp,g0)
    duel::confirm_cards(d, a.tp, g0);
    // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,0,LOCATION_HAND,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, 0, LOCATION_HAND, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        // local tc=sg:GetFirst()
        let tc = group::get_first(d, &sg).nil();
        // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
        // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1)
        card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // if Duel.IsTurnPlayer(1-tp) and Duel.IsPhase(PHASE_END) then
        if duel::is_turn_player(d, 1 - a.tp) && duel::is_phase(d, PHASE_END) {
            // e1:SetLabel(Duel.GetTurnCount())
            let tcount = duel::get_turn_count(d, None) as i64;
            effect::set_label(d, e1, &[tcount]);
            // e1:SetReset(RESET_PHASE|PHASE_END|RESET_OPPO_TURN,2)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 2);
        } else {
            // e1:SetLabel(0)
            effect::set_label(d, e1, &[0]);
            // e1:SetReset(RESET_PHASE|PHASE_END|RESET_OPPO_TURN)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 1);
        }
        // e1:SetLabelObject(tc)
        effect::set_label_object(d, e1, Value::Card(tc));
        // e1:SetCondition(s.retcon)
        effect::set_condition(d, e1, Some(condition(retcon)));
        // e1:SetOperation(s.retop)
        effect::set_operation(d, e1, Some(operation(retop)));
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    }
    // Duel.ShuffleHand(1-tp)
    duel::shuffle_hand(d, 1 - a.tp);
}
/// `s.retcon(e,tp,eg,ep,ev,re,r,rp)`
fn retcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // if tc:GetFlagEffect(id)==0 then
    if card::get_flag_effect(d, tc, ID) == 0 {
        // e:Reset()
        effect::reset(d, a.e);
        // return false
        false
    } else {
        // return Duel.IsTurnPlayer(1-tp) and Duel.GetTurnCount()~=e:GetLabel()
        duel::is_turn_player(d, 1 - a.tp) && duel::get_turn_count(d, None) as i64 != effect::get_label(d, a.e)
    }
}
/// `s.retop(e,tp,eg,ep,ev,re,r,rp)`
fn retop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // Duel.SendtoHand(tc,1-tp,REASON_EFFECT)
    duel::send_to_hand(d, tc, Some(1 - a.tp), REASON_EFFECT, None);
}
