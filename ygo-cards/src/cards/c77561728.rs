//! Disturbance Strategy
use super::prelude::*;

pub const ID: u32 = 77561728;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TODECK+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_TOHAND)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_TOHAND));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(1-tp)
    //     and Duel.IsExistingMatchingCard(Card.IsAbleToDeck,tp,0,LOCATION_HAND,1,nil) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, 1 - a.tp, 0)
            && duel::is_existing_matching_card(d, Some(&card::is_able_to_deck), a.tp, 0, LOCATION_HAND, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,1-tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, 1 - a.tp, LOCATION_HAND as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_HAND)
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_HAND);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
    let n = g.len();
    duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    // Duel.ShuffleDeck(1-tp)
    duel::shuffle_deck(d, 1 - a.tp);
    // Duel.BreakEffect()
    duel::break_effect(d);
    // Duel.Draw(1-tp,#g,REASON_EFFECT)
    duel::draw(d, 1 - a.tp, n as u32, REASON_EFFECT);
}
