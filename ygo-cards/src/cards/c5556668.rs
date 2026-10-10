//! Exchange
use super::prelude::*;

pub const ID: u32 = 5556668;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
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
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)>0
    //   and Duel.IsExistingMatchingCard(nil,tp,LOCATION_HAND,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        if duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND) <= 0 {
            return false;
        }
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, None, a.tp, LOCATION_HAND, 0, 1, h.into());
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g1=Duel.GetFieldGroup(tp,LOCATION_HAND,0)
    let g1 = duel::get_field_group(d, tp, LOCATION_HAND, 0);
    // local g2=Duel.GetFieldGroup(tp,0,LOCATION_HAND)
    let g2 = duel::get_field_group(d, tp, 0, LOCATION_HAND);
    // if #g1==0 or #g2==0 then return end
    if g1.is_empty() || g2.is_empty() {
        return;
    }
    // Duel.ConfirmCards(tp,g2)
    duel::confirm_cards(d, tp, g2.clone());
    // Duel.ConfirmCards(1-tp,g1)
    duel::confirm_cards(d, 1 - tp, g1.clone());
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local ag1=g2:Select(tp,1,1,nil)
    let ag1 = group::select(d, &g2, tp, 1, 1, false, NO_EX).nil();
    // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_ATOHAND as u64);
    // local ag2=g1:Select(1-tp,1,1,nil)
    let ag2 = group::select(d, &g1, 1 - tp, 1, 1, false, NO_EX).nil();
    // Duel.SendtoHand(ag1,tp,REASON_EFFECT)
    duel::send_to_hand(d, ag1, Some(tp), REASON_EFFECT, None);
    // Duel.SendtoHand(ag2,1-tp,REASON_EFFECT)
    duel::send_to_hand(d, ag2, Some(1 - tp), REASON_EFFECT, None);
}
