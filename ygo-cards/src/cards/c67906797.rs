//! The Great Noodle Inversion
use super::prelude::*;

pub const ID: u32 = 67906797;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// local sc=Duel.GetMatchingGroup(Card.IsSequence,tp,LOCATION_DECK,0,nil,0):GetFirst()
fn seq_card(d: &mut Duel, tp: u8) -> Option<CardId> {
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_sequence(d, c, &[0])), tp, LOCATION_DECK, 0, NO_EX);
    group::get_first(d, &g)
}
// s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local sc=...
    let sc = seq_card(d, a.tp);
    // if chk==0 then return Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>=2 and sc and sc:IsAbleToHand() end
    if a.chk == Some(0) {
        if !(duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) >= 2) {
            return false;
        }
        return match sc {
            None => false,
            Some(sc) => card::is_able_to_hand(d, sc, None),
        };
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local sc=Duel.GetMatchingGroup(Card.IsSequence,tp,LOCATION_DECK,0,nil,0):GetFirst()
    let sc = seq_card(d, tp).nil();
    // if sc:IsAbleToHand() and Duel.SendtoHand(sc,nil,REASON_EFFECT)>0 then
    if card::is_able_to_hand(d, sc, None) && duel::send_to_hand(d, sc, None, REASON_EFFECT, None) > 0 {
        // local dg=Duel.GetFieldGroup(tp,LOCATION_DECK,0)
        let dg = duel::get_field_group(d, tp, LOCATION_DECK, 0);
        // if #dg<=1 then return end
        if dg.len() <= 1 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
        // local bg=dg:Select(tp,1,1,nil)
        let bg = group::select(d, &dg, tp, 1, 1, false, NO_EX).nil();
        // Duel.ShuffleDeck(tp)
        duel::shuffle_deck(d, tp);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.MoveToDeckBottom(bg)
        duel::move_to_deck_bottom(d, &bg);
    } else {
        // Duel.SendtoGrave(sc,REASON_RULE)
        duel::send_to_grave(d, sc, REASON_RULE, None, None);
    }
}
