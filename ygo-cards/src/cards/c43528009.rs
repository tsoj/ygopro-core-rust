//! Gold Moon Coin
use super::prelude::*;

pub const ID: u32 = 43528009;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetMatchingGroupCount(aux.TRUE,tp,LOCATION_HAND,0,e:GetHandler())>1
    //   and Duel.IsPlayerCanDraw(tp,2) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::get_matching_group_count(d, None, a.tp, LOCATION_HAND, 0, h.into()) > 1 && duel::is_player_can_draw(d, a.tp, 2);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,2)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 2);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetFieldGroup(tp,LOCATION_HAND,0)
    let g = duel::get_field_group(d, tp, LOCATION_HAND, 0);
    // if #g>=2 then
    if g.len() >= 2 {
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,0))
        duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 0));
        // local ag=g:Select(tp,2,2,nil)
        let ag = group::select(d, &g, tp, 2, 2, false, NO_EX).nil();
        // Duel.SendtoHand(ag,1-tp,REASON_EFFECT)
        duel::send_to_hand(d, ag.clone(), Some(1 - tp), REASON_EFFECT, None);
        // Duel.ConfirmCards(tp,ag)
        duel::confirm_cards(d, tp, ag);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // Duel.ShuffleHand(1-tp)
        duel::shuffle_hand(d, 1 - tp);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(tp,2,REASON_EFFECT)
        duel::draw(d, tp, 2, REASON_EFFECT);
    }
}
