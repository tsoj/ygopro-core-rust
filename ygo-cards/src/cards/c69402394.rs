//! Dark Scheme
//! 暗黒の謀略
use super::prelude::*;

pub const ID: u32 = 69402394;

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
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local h1=Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)
        let mut h1 = duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0);
        // if e:GetHandler():IsLocation(LOCATION_HAND) then h1=h1-1 end
        let h = effect::get_handler(d, a.e).nil();
        if card::is_location(d, h, LOCATION_HAND) {
            h1 -= 1;
        }
        // local h2=Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)
        let h2 = duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND);
        // local d1=Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)
        let d1 = duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0);
        // local d2=Duel.GetFieldGroupCount(tp,0,LOCATION_DECK)
        let d2 = duel::get_field_group_count(d, a.tp, 0, LOCATION_DECK);
        // return h1>1 and h2>1 and d1>1 and d2>1
        return h1 > 1 && h2 > 1 && d1 > 1 && d2 > 1;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,PLAYER_ALL,2)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, PLAYER_ALL as u8, 2);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)<2 or Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)<2 then return end
    if duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0) < 2 || duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND) < 2 {
        return;
    }
    // if Duel.SelectYesNo(1-tp,aux.Stringid(id,0)) then
    if duel::select_yes_no(d, 1 - a.tp, aux::stringid(ID, 0)) {
        // Duel.DiscardHand(1-tp,aux.TRUE,1,1,REASON_EFFECT|REASON_DISCARD,nil)
        duel::discard_hand(d, 1 - a.tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
        // if Duel.IsChainDisablable(0) then
        if duel::is_chain_disablable(d, 0) {
            // Duel.NegateEffect(0)
            duel::negate_effect(d, 0);
            // return
            return;
        }
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DISCARD)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DISCARD as u64);
    // local g1=Duel.SelectMatchingCard(tp,aux.TRUE,tp,LOCATION_HAND,0,2,2,nil)
    let mut g1 = duel::select_matching_card(d, a.tp, None, a.tp, LOCATION_HAND, 0, 2, 2, false, NO_EX).nil();
    // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_DISCARD)
    duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_DISCARD as u64);
    // local g2=Duel.SelectMatchingCard(1-tp,aux.TRUE,1-tp,LOCATION_HAND,0,2,2,nil)
    let g2 = duel::select_matching_card(d, 1 - a.tp, None, 1 - a.tp, LOCATION_HAND, 0, 2, 2, false, NO_EX).nil();
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // Duel.SendtoGrave(g1,REASON_EFFECT|REASON_DISCARD)
    duel::send_to_grave(d, g1, REASON_EFFECT | REASON_DISCARD, None, None);
    // Duel.BreakEffect()
    duel::break_effect(d);
    // Duel.Draw(tp,2,REASON_EFFECT)
    duel::draw(d, a.tp, 2, REASON_EFFECT);
    // Duel.Draw(1-tp,2,REASON_EFFECT)
    duel::draw(d, 1 - a.tp, 2, REASON_EFFECT);
}
