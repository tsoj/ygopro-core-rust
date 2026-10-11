//! Amazing Pendulum
use super::prelude::*;

pub const ID: u32 = 37803970;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MAGICIAN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.GetFieldCard(tp,LOCATION_PZONE,0) and not Duel.GetFieldCard(tp,LOCATION_PZONE,1)
    duel::get_field_card(d, a.tp, LOCATION_PZONE, 0).is_none() && duel::get_field_card(d, a.tp, LOCATION_PZONE, 1).is_none()
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_MAGICIAN) and c:IsType(TYPE_PENDULUM) and c:IsAbleToHand()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_MAGICIAN as u16]) && card::is_type(d, c, TYPE_PENDULUM) && card::is_able_to_hand(d, c, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_EXTRA,0,nil)
        let g = duel::get_matching_group(d, Some(&thfilter), a.tp, LOCATION_EXTRA, 0, NO_EX);
        // return g:GetClassCount(Card.GetCode)>=2
        return group::get_class_count(d, &g, &|d, x| card::get_code(d, x)[0] as i64) >= 2;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,2,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 2, a.tp, LOCATION_EXTRA as i32);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_EXTRA,0,nil)
    let mut g = duel::get_matching_group(d, Some(&thfilter), a.tp, LOCATION_EXTRA, 0, NO_EX);
    // if g:GetClassCount(Card.GetCode)>=2 then
    if group::get_class_count(d, &g, &|d, x| card::get_code(d, x)[0] as i64) >= 2 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local g1=g:Select(tp,1,1,nil)
        let mut g1 = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        // g:Remove(Card.IsCode,nil,g1:GetFirst():GetCode())
        let code = card::get_code(d, group::get_first(d, &g1).nil())[0];
        group::remove(d, &mut g, &|d, x| card::is_code(d, x, &[code]), NO_EX);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local g2=g:Select(tp,1,1,nil)
        let g2 = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        // g1:Merge(g2)
        group::add_card(&mut g1, &g2);
        // Duel.SendtoHand(g1,nil,REASON_EFFECT)
        duel::send_to_hand(d, g1.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g1)
        duel::confirm_cards(d, 1 - a.tp, g1);
    }
}
