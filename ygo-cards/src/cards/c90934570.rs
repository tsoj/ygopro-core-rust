//! Infestation Terminus
use super::prelude::*;

pub const ID: u32 = 90934570;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_LSWARM], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // remove & tohand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER));
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.rmfilter(c)`
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_LSWARM) and c:IsAbleToRemove()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_LSWARM as u16]) && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.rmfilter,tp,LOCATION_MZONE,0,1,nil)
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&rmfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
            // and Duel.IsExistingTarget(Card.IsAbleToHand,tp,0,LOCATION_ONFIELD,2,nil) end
            && duel::is_existing_target(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, 0, LOCATION_ONFIELD, 2, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g1=Duel.SelectTarget(tp,s.rmfilter,tp,LOCATION_MZONE,0,1,1,nil)
    let g1 = duel::select_target(d, a.tp, Some(&rmfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g1,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g1, 1, 0, 0);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    // local g2=Duel.SelectTarget(tp,Card.IsAbleToHand,tp,0,LOCATION_ONFIELD,2,2,nil)
    let g2 = duel::select_target(
        d,
        a.tp,
        Some(&|d, c| card::is_able_to_hand(d, c, None)),
        a.tp,
        0,
        LOCATION_ONFIELD,
        2,
        2,
        false,
        NO_EX,
    );
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g2,#g2,0,0)
    let n = g2.clone().nil().len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g2, n, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local ex,g1=Duel.GetOperationInfo(0,CATEGORY_REMOVE)
    let g1 = duel::get_operation_info(d, 0, CATEGORY_REMOVE).and_then(|x| x.op_cards);
    // local ex,g2=Duel.GetOperationInfo(0,CATEGORY_TOHAND)
    let g2 = duel::get_operation_info(d, 0, CATEGORY_TOHAND).and_then(|x| x.op_cards);
    // if g1:GetFirst():IsRelateToEffect(e) and g1:GetFirst():IsFaceup() and Duel.Remove(g1,POS_FACEUP,REASON_EFFECT)~=0 then
    let tc1 = group::get_first(d, &g1.clone().nil()).nil();
    if card::is_relate_to_effect(d, tc1, a.e)
        && card::is_faceup(d, tc1)
        && duel::remove(d, g1.clone().nil(), POS_FACEUP as u8, REASON_EFFECT, None, None) != 0
    {
        // local hg=g2:Filter(Card.IsRelateToEffect,nil,e)
        let hg = group::filter(d, &g2.clone().nil(), &|d, c| card::is_relate_to_effect(d, c, a.e), NO_EX);
        // Duel.SendtoHand(hg,nil,REASON_EFFECT)
        duel::send_to_hand(d, hg, None, REASON_EFFECT, None);
    }
}
