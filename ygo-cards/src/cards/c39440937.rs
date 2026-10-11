//! Intercept Wave
use super::prelude::*;

pub const ID: u32 = 39440937;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_POSITION)
    effect::set_category(d, e1, CATEGORY_POSITION);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsPosition(POS_FACEUP_ATTACK) and c:IsType(TYPE_SYNCHRO) and c:IsCanChangePosition()
    card::is_position(d, c, POS_FACEUP_ATTACK) && card::is_type(d, c, TYPE_SYNCHRO) && card::is_can_change_position(d, c)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(g), n, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if #g>0 and Duel.ChangePosition(g,POS_FACEUP_DEFENSE)~=0 then
    if !g.is_empty() && duel::change_position(d, g, POS_FACEUP_DEFENSE as u8, None, None, None, false) != 0 {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // e1:SetCondition(s.tdcon)
        effect::set_condition(d, e1, Some(condition(tdcon)));
        // e1:SetOperation(s.tdop)
        effect::set_operation(d, e1, Some(operation(tdop)));
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    }
}
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_SYNCHRO) and c:IsAbleToExtra()
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_SYNCHRO) && card::is_able_to_extra(d, c)
}
fn tdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.tdfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    duel::is_existing_matching_card(d, Some(&tdfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
}
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.tdfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&tdfilter), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.SendtoDeck(g,nil,SEQ_DECKTOP,REASON_EFFECT)
    duel::send_to_deck(d, g, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
}
