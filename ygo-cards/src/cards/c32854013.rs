//! Super Rush Recklessly
use super::prelude::*;

pub const ID: u32 = 32854013;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_TODECK);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.FaceupFilter(Card.IsRace,RACE_BEAST),tp,LOCATION_MZONE,0,1,nil)
    //   and Duel.IsExistingTarget(Card.IsAbleToDeck,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&aux::faceup_filter(|d, c| card::is_race(d, c, RACE_BEAST))), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&|d, c| card::is_able_to_deck(d, c)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g1=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsRace,RACE_BEAST),tp,LOCATION_MZONE,0,1,1,nil)
    let g1 = duel::select_target(d, tp, Some(&aux::faceup_filter(|d, c| card::is_race(d, c, RACE_BEAST))), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // e:SetLabelObject(g1:GetFirst())
    let first = group::get_first(d, &g1.clone().nil());
    effect::set_label_object(d, e, first.map_or(Value::Nil, Value::Card));
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g2=Duel.SelectTarget(tp,Card.IsAbleToDeck,tp,0,LOCATION_MZONE,1,1,nil)
    let g2 = duel::select_target(d, tp, Some(&|d, c| card::is_able_to_deck(d, c)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g1,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g1, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g2,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, g2, 1, 0, 0);
    true
}
/// `s.activate(e)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc1=e:GetLabelObject()
    let tc1 = effect::get_label_object(d, a.e).as_card();
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    // local tc2=g:GetFirst()
    let order = group::iter(d, &g);
    let mut tc2 = order.first().copied();
    // if tc2==tc1 then tc2=g:GetNext() end
    if tc2 == tc1 {
        tc2 = order.get(1).copied();
    }
    // if tc1:IsRelateToEffect(e) and tc1:IsFaceup() and Duel.Destroy(tc1,REASON_EFFECT)~=0 and tc2:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc1.nil(), a.e)
        && card::is_faceup(d, tc1.nil())
        && duel::destroy(d, tc1.nil(), REASON_EFFECT, None, None) != 0
        && card::is_relate_to_effect(d, tc2.nil(), a.e)
    {
        // Duel.SendtoDeck(tc2,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, tc2.nil(), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
