//! Compulsory Escape Device
use super::prelude::*;

pub const ID: u32 = 48216773;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_TODECK);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E as u32));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToDeck,tp,LOCATION_MZONE,0,1,nil)
    //   and Duel.IsExistingMatchingCard(Card.IsAbleToDeck,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&card::is_able_to_deck), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&card::is_able_to_deck), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,2,PLAYER_ALL,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 2, PLAYER_ALL as u8, LOCATION_MZONE as i32);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local dg1=Duel.SelectMatchingCard(tp,Card.IsAbleToDeck,tp,LOCATION_MZONE,0,1,1,nil)
    let mut dg1 = duel::select_matching_card(d, tp, Some(&card::is_able_to_deck), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_TODECK as u64);
    // local dg2=Duel.SelectMatchingCard(1-tp,Card.IsAbleToDeck,1-tp,LOCATION_MZONE,0,1,1,nil)
    let dg2 = duel::select_matching_card(d, 1 - tp, Some(&card::is_able_to_deck), 1 - tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // dg1:Merge(dg2)
    group::add_card(&mut dg1, &dg2);
    // Duel.HintSelection(dg1)
    duel::hint_selection(d, dg1.clone(), true);
    // Duel.SendtoDeck(dg1,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
    duel::send_to_deck(d, dg1, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
}
