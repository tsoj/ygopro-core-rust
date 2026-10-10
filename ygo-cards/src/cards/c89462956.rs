//! Dealer's Choice
use super::prelude::*;

pub const ID: u32 = 89462956;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_HANDES+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_HANDES + CATEGORY_DRAW);
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
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) and Duel.IsPlayerCanDraw(1-tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1) && duel::is_player_can_draw(d, 1 - a.tp, 1);
    }
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,PLAYER_ALL,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, PLAYER_ALL as u8, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,PLAYER_ALL,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, PLAYER_ALL as u8, 1);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // Duel.ShuffleDeck(tp)
    duel::shuffle_deck(d, a.tp);
    // Duel.ShuffleDeck(1-tp)
    duel::shuffle_deck(d, 1 - a.tp);
    // local h1=Duel.Draw(tp,1,REASON_EFFECT)
    let h1 = duel::draw(d, a.tp, 1, REASON_EFFECT);
    // local h2=Duel.Draw(1-tp,1,REASON_EFFECT)
    let h2 = duel::draw(d, 1 - a.tp, 1, REASON_EFFECT);
    // if h1>0 or h2>0 then Duel.BreakEffect() end
    if h1 > 0 || h2 > 0 {
        duel::break_effect(d);
    }
    // if h1>0 then
    if h1 > 0 {
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, a.tp);
        // Duel.DiscardHand(tp,aux.TRUE,1,1,REASON_EFFECT|REASON_DISCARD)
        duel::discard_hand(d, a.tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
    }
    // if h2>0 then
    if h2 > 0 {
        // Duel.ShuffleHand(1-tp)
        duel::shuffle_hand(d, 1 - a.tp);
        // Duel.DiscardHand(1-tp,aux.TRUE,1,1,REASON_EFFECT|REASON_DISCARD)
        duel::discard_hand(d, 1 - a.tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
    }
}
